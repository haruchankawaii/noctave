use harmony_engine::{generate, midi, style, transpose, Progression, ENGINE_VERSION};
use serde::Deserialize;
use serde_json::json;
use std::{
    io::{Read, Write},
    net::TcpStream,
    process::Command,
    time::Duration,
};
use tiny_http::{Header, Method, Response, Server, StatusCode};

const INDEX: &str = include_str!("../../../ui/index.html");
const SCRIPT: &str = include_str!(concat!(env!("OUT_DIR"), "/app.js"));
const CSS: &str = include_str!("../../../ui/styles.css");
const LOGO: &str = include_str!("../../../ui/mark.svg");
const MAX_BODY: usize = 262_144;

fn header(name: &str, value: &str) -> Header {
    Header::from_bytes(name, value).expect("valid static HTTP header")
}
fn respond(request: tiny_http::Request, status: u16, kind: &str, body: Vec<u8>) {
    let response=Response::from_data(body).with_status_code(StatusCode(status))
        .with_header(header("Content-Type",kind))
        .with_header(header("Cache-Control","no-store"))
        .with_header(header("X-Content-Type-Options","nosniff"))
        .with_header(header("Referrer-Policy","no-referrer"))
        .with_header(header("Content-Security-Policy","default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self'; media-src 'self' blob:; object-src 'none'; base-uri 'none'; frame-ancestors 'none'"));
    if let Err(error) = request.respond(response) {
        eprintln!("Response disconnected: {error}");
    }
}
fn error(request: tiny_http::Request, status: u16, message: &str) {
    respond(
        request,
        status,
        "application/json; charset=utf-8",
        json!({"error":message}).to_string().into_bytes(),
    );
}
fn get_header<'a>(request: &'a tiny_http::Request, name: &str) -> Option<&'a str> {
    request
        .headers()
        .iter()
        .find(|h| h.field.as_str().as_str().eq_ignore_ascii_case(name))
        .map(|h| h.value.as_str())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TransposeRequest {
    progression: Progression,
    key: String,
}

fn api(path: &str, body: &[u8]) -> Result<(&'static str, Vec<u8>), String> {
    let progression = match path {
        "/api/generate" => generate(serde_json::from_slice(body).map_err(|e| e.to_string())?)?,
        "/api/transpose" => {
            let request: TransposeRequest =
                serde_json::from_slice(body).map_err(|e| e.to_string())?;
            transpose(&request.progression, &request.key)?
        }
        "/api/restore" => {
            let progression: Progression =
                serde_json::from_slice(body).map_err(|e| e.to_string())?;
            if progression.engine_version != ENGINE_VERSION {
                return Err("This sketch was made with a different engine version".into());
            }
            transpose(&progression, &progression.request.key)?
        }
        "/api/midi" => {
            let progression: Progression =
                serde_json::from_slice(body).map_err(|e| e.to_string())?;
            let canonical = transpose(&progression, &progression.request.key)?;
            return Ok(("audio/midi", midi::export(&canonical)?));
        }
        _ => return Err("Unknown API route".into()),
    };
    Ok((
        "application/json; charset=utf-8",
        serde_json::to_vec(&progression).map_err(|e| e.to_string())?,
    ))
}

pub fn serve(port: u16, open: bool) -> Result<(), String> {
    let host = format!("127.0.0.1:{port}");
    let origin = format!("http://{host}");
    let server = match Server::http(&host) {
        Ok(server) => server,
        Err(_) if open && existing_noctave(&host) => return launch(&origin),
        Err(error) => {
            let message =
                format!("Cannot open {host}: {error}. Try another port with --port 48732.");
            return Err(message);
        }
    };
    println!("Noctave {ENGINE_VERSION}\nStudio: {origin}\nAll processing stays on this device.\nUse Quit in the studio or Ctrl+C to close the server.");
    if open {
        if let Err(e) = launch(&origin) {
            eprintln!("Open {origin} in your browser ({e})");
        }
    }
    for mut request in server.incoming_requests() {
        // Exact loopback Host validation prevents DNS rebinding. No CORS headers.
        if get_header(&request, "host") != Some(host.as_str()) {
            error(request, 403, "Invalid local host");
            continue;
        }
        let path = request.url().split('?').next().unwrap_or("/").to_string();
        if request.method() == &Method::Get {
            let (kind,body)=match path.as_str(){
                "/"|"/index.html"=>("text/html; charset=utf-8",INDEX.as_bytes().to_vec()),
                "/app.js"=>("text/javascript; charset=utf-8",SCRIPT.as_bytes().to_vec()),
                "/styles.css"=>("text/css; charset=utf-8",CSS.as_bytes().to_vec()),
                "/mark.svg"|"/favicon.ico"=>("image/svg+xml",LOGO.as_bytes().to_vec()),
                "/api/info"=>("application/json; charset=utf-8",json!({"application":"noctave","version":ENGINE_VERSION,"styles":style::definitions(),"defaults":harmony_engine::Request::default()}).to_string().into_bytes()),
                _=>{error(request,404,"Not found");continue;},
            };
            respond(request, 200, kind, body);
            continue;
        }
        if request.method() != &Method::Post {
            error(request, 405, "Use GET or POST");
            continue;
        }
        if get_header(&request, "origin").is_some_and(|value| value != origin)
            || get_header(&request, "x-noctave") != Some("1")
        {
            error(
                request,
                403,
                "Only same-origin Noctave requests are accepted",
            );
            continue;
        }
        if !get_header(&request, "content-type")
            .is_some_and(|value| value.starts_with("application/json"))
        {
            error(request, 415, "JSON request required");
            continue;
        }
        if request.body_length().is_some_and(|len| len > MAX_BODY) {
            error(request, 413, "Request is too large");
            continue;
        }
        let mut body = vec![];
        if request
            .as_reader()
            .take((MAX_BODY + 1) as u64)
            .read_to_end(&mut body)
            .is_err()
        {
            error(request, 400, "Cannot read request");
            continue;
        }
        if body.len() > MAX_BODY {
            error(request, 413, "Request is too large");
            continue;
        }
        if path == "/api/quit" {
            respond(
                request,
                200,
                "application/json",
                b"{\"closed\":true}".to_vec(),
            );
            break;
        }
        if ![
            "/api/generate",
            "/api/transpose",
            "/api/restore",
            "/api/midi",
        ]
        .contains(&path.as_str())
        {
            error(request, 404, "Unknown API route");
            continue;
        }
        match api(&path, &body) {
            Ok((kind, data)) => respond(request, 200, kind, data),
            Err(message) => error(request, 400, &message),
        }
    }
    Ok(())
}

fn existing_noctave(host: &str) -> bool {
    let Ok(address) = host.parse() else {
        return false;
    };
    let Ok(mut stream) = TcpStream::connect_timeout(&address, Duration::from_millis(400)) else {
        return false;
    };
    let _ = stream.set_read_timeout(Some(Duration::from_millis(700)));
    let _ = stream.set_write_timeout(Some(Duration::from_millis(400)));
    if write!(
        stream,
        "GET /api/info HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n"
    )
    .is_err()
    {
        return false;
    }
    let mut response = String::new();
    if stream.take(65536).read_to_string(&mut response).is_err() {
        return false;
    }
    let Some((_, body)) = response.split_once("\r\n\r\n") else {
        return false;
    };
    serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .is_some_and(|value| {
            value["application"] == "noctave" && value["version"] == ENGINE_VERSION
        })
}

#[cfg(target_os = "windows")]
fn launch(url: &str) -> Result<(), String> {
    for folder in ["ProgramFiles(x86)", "ProgramFiles"] {
        if let Some(root) = std::env::var_os(folder) {
            let edge = std::path::Path::new(&root).join("Microsoft/Edge/Application/msedge.exe");
            if edge.is_file() {
                Command::new(edge)
                    .args([format!("--app={url}"), "--window-size=1480,1000".into()])
                    .spawn()
                    .map_err(|e| e.to_string())?;
                return Ok(());
            }
        }
    }
    Command::new("rundll32.exe")
        .args(["url.dll,FileProtocolHandler", url])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}
#[cfg(target_os = "macos")]
fn launch(url: &str) -> Result<(), String> {
    Command::new("open")
        .arg(url)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}
#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
fn launch(url: &str) -> Result<(), String> {
    Command::new("xdg-open")
        .arg(url)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod unit {
    use super::*;
    #[test]
    fn malformed_api_requests_fail() {
        assert!(api("/api/generate", b"{\"length\":0}").is_err());
        assert!(api("/api/generate", b"{\"unknown\":5}").is_err());
        assert!(api("/api/transpose", b"{}").is_err());
    }
    #[test]
    fn generation_and_canonical_midi() {
        let (_, data) = api("/api/generate", b"{\"length\":2}").unwrap();
        let mut result: Progression = serde_json::from_slice(&data).unwrap();
        result.chords[0].midi = vec![200];
        let body = serde_json::to_vec(&result).unwrap();
        let (kind, bytes) = api("/api/midi", &body).unwrap();
        assert_eq!(kind, "audio/midi");
        assert_eq!(&bytes[..4], b"MThd");
    }
}
