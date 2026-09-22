mod server;
use harmony_engine::{generate, midi, Request, ENGINE_VERSION};
use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
    process,
};

const HELP:&str="Noctave — a local harmonic sketchbook\n\n  noctave                         Open the studio\n  noctave --no-open                Run without opening a window\n  noctave --port 48731             Choose the local port\n  noctave --generate --seed 42     Print a progression\n  noctave --generate --format json Export a reproducible JSON sketch\n  noctave --request sketch.json    Generate from a request JSON file\n  noctave --generate --format midi --output sketch.mid\n\nGeneration options:\n  --key D --mode major --style post-rock --length 8 --seed 29481932\n  --mood dreamy --complexity balanced --creativity 35\n  --energy slow-build --cadence open --tempo 88 --beats-per-chord 4\n  --region global --era any --harmonic-trait none\n  --format text|json|midi --output PATH\n\nAll processing is local. Beta profiles are experimental.\n";

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.iter().any(|v| v == "--help" || v == "-h") {
        print!("{HELP}");
        return Ok(());
    }
    if args.iter().any(|v| v == "--version") {
        println!("Noctave {ENGINE_VERSION}");
        return Ok(());
    }
    let mut request = serde_json::to_value(Request::default()).map_err(|e| e.to_string())?;
    let mut generation = false;
    let mut open = true;
    let mut port = 48731u16;
    let mut format = "text".to_string();
    let mut output: Option<PathBuf> = None;
    let mut overrides = vec![];
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "--generate" => {
                generation = true;
                i += 1;
                continue;
            }
            "--no-open" => {
                open = false;
                i += 1;
                continue;
            }
            _ => {}
        }
        let value = args
            .get(i + 1)
            .ok_or_else(|| format!("Missing value for {arg}"))?;
        match arg.as_str() {
            "--port" => {
                port = value.parse().map_err(|_| "Invalid port")?;
                if port == 0 {
                    return Err("Port must be 1–65535".into());
                }
            }
            "--format" => format = value.clone(),
            "--output" => output = Some(value.into()),
            "--request" => {
                let bytes = fs::read(value).map_err(|e| format!("Cannot read request: {e}"))?;
                if bytes.len() > 262_144 {
                    return Err("Request file is too large".into());
                }
                request = serde_json::from_slice(&bytes)
                    .map_err(|e| format!("Invalid request JSON: {e}"))?;
                if let Some(inner) = request.get("request") {
                    request = inner.clone();
                }
                generation = true;
            }
            "--key" | "--mode" | "--style" | "--region" | "--era" | "--mood" | "--complexity"
            | "--energy" | "--cadence" | "--harmonic-trait" => overrides.push((
                arg[2..].replace('-', "_"),
                serde_json::Value::String(value.clone()),
            )),
            "--length" | "--seed" | "--creativity" | "--tempo" | "--beats-per-chord" => {
                let number = value
                    .parse::<u64>()
                    .map_err(|_| format!("{arg} requires a non-negative integer"))?;
                overrides.push((arg[2..].replace('-', "_"), number.into()));
            }
            _ => return Err(format!("Unknown option: {arg}. Use --help.")),
        }
        i += 2;
    }
    if !request.is_object() {
        return Err("Request must be a JSON object".into());
    }
    for (key, value) in overrides {
        request[&key] = value;
    }
    if generation {
        let request: Request = serde_json::from_value(request).map_err(|e| e.to_string())?;
        let result = generate(request)?;
        let data = match format.as_str() {
            "json" => serde_json::to_vec_pretty(&result).map_err(|e| e.to_string())?,
            "midi" => {
                if output.is_none() {
                    return Err("MIDI export requires --output filename.mid".into());
                }
                midi::export(&result)?
            }
            "text" => format!(
                "Noctave {} | {} {} | {} | seed {}\n{}\n{}\n",
                ENGINE_VERSION,
                result.request.key,
                result.request.mode.label(),
                result.profile.name,
                result.request.seed,
                result
                    .chords
                    .iter()
                    .map(|c| c.name.as_str())
                    .collect::<Vec<_>>()
                    .join("  →  "),
                result
                    .chords
                    .iter()
                    .map(|c| c.roman.as_str())
                    .collect::<Vec<_>>()
                    .join("  →  ")
            )
            .into_bytes(),
            _ => return Err("Format must be text, json, or midi".into()),
        };
        if let Some(path) = output {
            fs::write(path, data).map_err(|e| e.to_string())?;
        } else {
            io::stdout().write_all(&data).map_err(|e| e.to_string())?;
        }
        Ok(())
    } else {
        server::serve(port, open)
    }
}
fn main() {
    if let Err(error) = run() {
        eprintln!("Noctave: {error}");
        process::exit(1);
    }
}
