import { resolve } from "node:path";

const root = resolve(import.meta.dir, "..");
const frontendPort = Number(process.env.NOCTAVE_DEV_PORT ?? 5173);
const enginePort = Number(process.env.NOCTAVE_ENGINE_PORT ?? 48732);
for (const port of [frontendPort, enginePort])
  if (!Number.isInteger(port) || port < 1024 || port > 65535)
    throw new Error("Development ports must be integers from 1024 to 65535.");
if (frontendPort === enginePort)
  throw new Error("Frontend and engine need separate ports.");
const host = `127.0.0.1:${frontendPort}`,
  origin = `http://${host}`,
  backend = `http://127.0.0.1:${enginePort}`;
const build = Bun.spawn(["cargo", "build", "-p", "noctave", "--locked"], {
  cwd: root,
  stdout: "inherit",
  stderr: "inherit",
});
if ((await build.exited) !== 0) process.exit(1);
const executable = resolve(
  root,
  `target/debug/noctave${process.platform === "win32" ? ".exe" : ""}`,
);
const engine = Bun.spawn(
  [executable, "--no-open", "--port", String(enginePort)],
  { cwd: root, stdout: "inherit", stderr: "inherit" },
);
let server: ReturnType<typeof Bun.serve> | undefined;
const shutdown = () => {
  server?.stop(true);
  if (engine.exitCode === null) engine.kill();
};
process.on("exit", shutdown);
process.on("SIGINT", () => process.exit(0));
process.on("SIGTERM", () => process.exit(0));
for (let tries = 0; ; tries++) {
  if (engine.exitCode !== null)
    throw new Error(
      "Rust engine could not start. Check whether its port is already used.",
    );
  try {
    if ((await fetch(`${backend}/api/info`)).ok) break;
  } catch {}
  if (tries >= 60) throw new Error("Rust engine did not become ready.");
  await Bun.sleep(100);
}
const headers = {
  "X-Content-Type-Options": "nosniff",
  "Cache-Control": "no-store",
  "Content-Security-Policy":
    "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self'; media-src 'self' blob:; object-src 'none'; base-uri 'none'; frame-ancestors 'none'",
};
server = Bun.serve({
  hostname: "127.0.0.1",
  port: frontendPort,
  async fetch(request) {
    if (request.headers.get("host") !== host)
      return new Response("Invalid local host", { status: 403 });
    const path = new URL(request.url).pathname;
    if (path.startsWith("/api/")) {
      if (request.method !== "GET" && request.method !== "POST")
        return new Response("Method not allowed", { status: 405 });
      if (
        request.method === "POST" &&
        (request.headers.get("origin") !== origin ||
          request.headers.get("x-noctave") !== "1")
      )
        return new Response("Same-origin requests required", { status: 403 });
      try {
        const response = await fetch(`${backend}${path}`, {
          method: request.method,
          headers:
            request.method === "POST"
              ? {
                  "Content-Type": "application/json",
                  "X-Noctave": "1",
                  Origin: backend,
                }
              : {},
          body:
            request.method === "POST" ? await request.arrayBuffer() : undefined,
        });
        if (path === "/api/quit" && response.ok)
          setTimeout(() => process.exit(0), 300);
        return new Response(response.body, {
          status: response.status,
          headers: {
            ...headers,
            "Content-Type":
              response.headers.get("content-type") ?? "application/json",
          },
        });
      } catch {
        return Response.json(
          { error: "The Rust development engine is unavailable." },
          { status: 502, headers },
        );
      }
    }
    if (request.method !== "GET")
      return new Response("Method not allowed", { status: 405 });
    if (path === "/app.js") {
      const result = await Bun.build({
        entrypoints: [resolve(root, "ui/app.ts")],
        target: "browser",
        format: "esm",
      });
      if (!result.success)
        return new Response(result.logs.map(String).join("\n"), {
          status: 500,
          headers,
        });
      return new Response(result.outputs[0], {
        headers: { ...headers, "Content-Type": "text/javascript" },
      });
    }
    const assets: Record<string, [string, string]> = {
      "/": ["index.html", "text/html"],
      "/index.html": ["index.html", "text/html"],
      "/styles.css": ["styles.css", "text/css"],
      "/mark.svg": ["mark.svg", "image/svg+xml"],
      "/favicon.ico": ["mark.svg", "image/svg+xml"],
    };
    const asset = assets[path];
    if (!asset) return new Response("Not found", { status: 404 });
    return new Response(Bun.file(resolve(root, "ui", asset[0])), {
      headers: { ...headers, "Content-Type": `${asset[1]}; charset=utf-8` },
    });
  },
});
console.log(
  `\nBun TypeScript development studio: ${origin}\nEdit ui/*.ts or CSS and reload to see changes. Ctrl+C closes both processes.\n`,
);
