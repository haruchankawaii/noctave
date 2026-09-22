import { resolve } from "node:path";

const flag = Bun.argv.indexOf("--outdir");
const outdir = resolve(flag >= 0 ? Bun.argv[flag + 1] : "target/ui");
const result = await Bun.build({
  entrypoints: [resolve(import.meta.dir, "../ui/app.ts")],
  outdir,
  target: "browser",
  format: "esm",
  minify: true,
  naming: "app.js",
});
if (!result.success) {
  for (const log of result.logs) console.error(log);
  process.exit(1);
}
console.log(`Bundled TypeScript with Bun → ${outdir}`);
