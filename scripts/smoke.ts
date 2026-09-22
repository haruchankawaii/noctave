import { resolve } from "node:path";
import type { Progression, GenerationRequest } from "../ui/model.ts";

const root = resolve(import.meta.dir, ".."),
  port = 48741,
  base = `http://127.0.0.1:${port}`;
const executable = resolve(
  root,
  `target/release/noctave${process.platform === "win32" ? ".exe" : ""}`,
);
const child = Bun.spawn([executable, "--no-open", "--port", String(port)], {
  cwd: root,
  stdout: "ignore",
  stderr: "inherit",
});
const headers = {
  "Content-Type": "application/json",
  "X-Noctave": "1",
  Origin: base,
};
let assertions = 0;
function check(condition: unknown, message: string): asserts condition {
  if (!condition) throw new Error(message);
  assertions++;
}
async function post(path: string, body: unknown): Promise<Response> {
  return fetch(base + path, {
    method: "POST",
    headers,
    body: JSON.stringify(body),
    signal: AbortSignal.timeout(30000),
  });
}
async function json(path: string, body: unknown): Promise<Progression> {
  const response = await post(path, body);
  check(response.ok, `HTTP failure ${path}: ${response.status}`);
  return response.json();
}
try {
  let ready = false;
  for (let attempt = 0; attempt < 60; attempt++) {
    if (child.exitCode !== null)
      throw new Error(
        "Smoke-test engine exited before readiness. Port 48741 may be in use.",
      );
    try {
      const response = await fetch(`${base}/api/info`);
      if (response.ok) {
        ready = true;
        break;
      }
    } catch {}
    await Bun.sleep(100);
  }
  check(ready, "Engine readiness timeout");
  const page = await fetch(base);
  check(page.ok, "Studio HTML");
  check((await page.text()).includes("Find your next feeling."), "Embedded UI");
  check(
    page.headers
      .get("content-security-policy")
      ?.includes("frame-ancestors 'none'"),
    "Content security policy",
  );
  const script = await fetch(`${base}/app.js`);
  check(
    script.ok && (await script.text()).length > 10000,
    "Bun-built JavaScript bundle is embedded",
  );
  const request = { seed: 42, length: 8 };
  const a = await json("/api/generate", request),
    b = await json("/api/generate", request);
  check(JSON.stringify(a) === JSON.stringify(b), "Deterministic HTTP result");
  const locked: GenerationRequest = {
    ...a.request,
    seed: 123,
    locks: a.chords.map((c, i) => (i % 2 === 0 ? c.chord : null)),
  };
  const c = await json("/api/generate", locked);
  for (let i = 0; i < 8; i += 2)
    check(
      JSON.stringify(c.chords[i].chord) === JSON.stringify(a.chords[i].chord),
      `Lock ${i}`,
    );
  const restored = await json("/api/restore", a);
  check(
    JSON.stringify(restored) === JSON.stringify(a),
    "Exact restore including explanations and score",
  );
  const transposed = await json("/api/transpose", {
    progression: a,
    key: "F#",
  });
  check(transposed.request.key === "F#", "Transpose key");
  check(
    transposed.chords.every((chord, i) => chord.roman === a.chords[i].roman),
    "Transpose preserves Roman structure",
  );
  const midi = await post("/api/midi", a),
    bytes = new Uint8Array(await midi.arrayBuffer());
  check(new TextDecoder().decode(bytes.slice(0, 4)) === "MThd", "MIDI header");
  check(
    new DataView(bytes.buffer).getUint32(18) === bytes.length - 22,
    "MIDI track size",
  );
  check(
    (await post("/api/generate", { length: 0 })).status === 400,
    "Invalid request rejected",
  );
  check(
    (
      await fetch(`${base}/api/generate`, {
        method: "POST",
        headers: { ...headers, Origin: "https://example.com" },
        body: "{}",
      })
    ).status === 403,
    "Cross-origin request rejected",
  );
  check(
    (
      await fetch(`${base}/api/generate`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: "{}",
      })
    ).status === 403,
    "Missing custom header rejected",
  );
  check(
    (
      await fetch(`${base}/api/generate`, {
        method: "POST",
        headers,
        body: " ".repeat(262145),
      })
    ).status === 413,
    "Oversize request rejected",
  );
  check(
    (await fetch(`${base}/../Cargo.toml`)).status === 404,
    "No source-file serving",
  );
  const durations: number[] = [];
  for (let seed = 1; seed <= 12; seed++) {
    const start = performance.now();
    await json("/api/generate", {
      seed,
      length: 16,
      complexity: "rich",
      creativity: 100,
    });
    durations.push(performance.now() - start);
  }
  durations.sort((x, y) => x - y);
  console.log(
    JSON.stringify(
      {
        assertions,
        largestRequest: "16 chords, rich, 100% creativity",
        generationMilliseconds: {
          median: Math.round(durations[6]),
          maximum: Math.round(durations[11]),
        },
      },
      null,
      2,
    ),
  );
} finally {
  if (child.exitCode === null) {
    try {
      await post("/api/quit", {});
    } catch {}
    await Promise.race([child.exited, Bun.sleep(1000)]);
    if (child.exitCode === null) child.kill();
  }
}
