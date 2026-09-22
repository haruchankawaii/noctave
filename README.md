# Noctave

**A local harmonic sketchbook.** Explore chord progressions with music theory,
weighted randomness, and a little room for serendipity.

Noctave **0.1.0-beta.1** is a free early beta. Its Rust engine runs entirely on
your computer. No account, network generation, telemetry, or AI model is involved.

## Run the Windows beta

If you have the portable package, extract it and double-click **Start Noctave.cmd**.
The standalone `noctave.exe` also works. No Rust installation is needed for the
portable build. The interface opens in an Edge app window, or your default browser.
No external fonts, libraries, or audio samples are fetched.

From this source checkout, double-click **Start-Noctave.cmd**, or run:

```sh
cargo run --release -p noctave
```

Building requires Bun 1.3.14+, the current stable Rust toolchain and platform C/C++ linker
(Visual Studio C++ Build Tools on Windows). If the executable is already running,
visit `http://127.0.0.1:48731`. Use the power button in the studio to quit the
engine; simply closing the browser window keeps the local engine running.

## Make your first sketch

1. Choose a style, key/mode, mood, complexity, and energy shape.
2. Click **Generate progression**. Each click explores a fresh seed.
3. Click a chord to audition it and inspect notes, voicing, and scoring reasons.
4. Lock the chords you like, then **Regenerate unlocked** to explore continuations.
5. Use **Transpose** to move the exact progression to another key.
6. **Save sketch** keeps a copy in this browser. Export JSON for a portable backup,
   MIDI for your DAW, or a chord sheet / copied Roman numerals for your notes.

The seed field's arrow regenerates with the seed you entered. Reproduction requires
the same settings, locks, engine version, and style profiles. Your session restores
automatically in the same browser and local port. Clearing browser data removes
local sketches, so keep JSON backups of important ideas.

Shortcuts: `G` generate, `Space` play/stop, `L` lock the selected chord,
`Ctrl/Cmd+S` save, `Ctrl/Cmd+Z` undo generation, arrows select chords, `Esc` close.
Shortcuts are inactive while typing. Playback stops when the page is hidden.

## What is in this beta

- Six Rock-family styles: Rock, Post-Rock, Math Rock, Shoegaze, Indie Rock,
  and Midwest Emo. Post-Rock also has exploratory Japan and 2000s modifiers.
- Major, natural minor, Dorian, Phrygian, Lydian, Mixolydian, and Locrian.
- 2–16 chords; composable triads, sevenths, added/extended tones, suspensions,
  parallel-mode borrowing, applied dominants, and optimized inversions.
- Reproducible seeded phrase search, open/resolved/loop endings, a tension target,
  common-tone / root-movement scoring, and actual rule explanations.
- Exact chord locks, whole-progression transposition, undo, local sketchbook,
  JSON import/export, text export, and single-track MIDI.
- Simple Web Audio audition with tempo, chord duration, volume, and looping.

All profiles are **experimental, unvalidated, and hand-authored**. They have zero
claimed corpus samples. Region/era choices are sound-design hypotheses. Musical
quality and perceptual style separation still need musician listening tests.
See [the research workflow](research/README.md).

## Deliberate early-beta boundaries

- One genre family; no artist imitation or verified regional classification.
- No VST/DAW plugin, melody/song composition, audio generation/export, or cloud sync.
- Browser-based desktop shell; no native installer, auto-update, or signed binary.
- MIDI is 4/4, acoustic piano, one track, and uniform duration per chord.
- Preview is a lightweight synthesizer, not a sampled acoustic instrument.
- Tension is a contextual heuristic; its target is a preference, not a guarantee.
- A resolved ending requires a mode-appropriate tonic unless the last chord is
  locked. Locks take priority over generation preferences. In Locrian that tonic
  is diminished and does not imply the same stability as a major/minor tonic.
- Native JSON sketches restore exact chord structures and inversions. The beta
  Roman parser supports the documented dialect, not arbitrary textbook notation
  or secondary-function slash notation such as V/V.
- No automatic research validation, huge corpus ingestion, or complete taxonomy.

## CLI

```sh
cargo run --release -p noctave -- --generate --key D --style post-rock --seed 42
cargo run --release -p noctave -- --generate --mode dorian --complexity rich --creativity 60
cargo run --release -p noctave -- --generate --format json --output sketch.json
cargo run --release -p noctave -- --generate --format midi --output sketch.mid
cargo run --release -p noctave -- --request sketch.json --format text
cargo run --release -p noctave -- --no-open --port 48732
```

`--request` accepts request JSON or an exported sketch's `request` field; it
**regenerates** from those inputs. Use the studio's JSON import to restore an
exact saved progression instead. `--help` lists every supported option.

## Development

```sh
bun install --frozen-lockfile
bun run dev
bun run typecheck
bun test ui
bun run format:check
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cargo run -p harmony-engine --example analyze_corpus -- research/example-corpus.json
bun run build
bun run test:smoke
```

The interface uses HTML/CSS and **strict TypeScript**, with **Bun** as the runtime,
bundler, package manager, and TypeScript test runner. No authored JavaScript files
or Node/npm runtime remain. Bun emits a browser-compatible bundle into ignored
build output, and Cargo embeds it in the executable automatically.

`bun run dev` opens a development server at `http://127.0.0.1:5173` with a Rust
backend at port 48732. Edit TypeScript/CSS and reload the page. For the production
executable, rebuild and restart after edits. Set `NOCTAVE_DEV_PORT` and
`NOCTAVE_ENGINE_PORT` to override development ports. Portable users need neither
Rust nor Bun installed. First-time development setup downloads dependencies;
`Cargo.lock` and `bun.lock` pin their versions.

Build a Windows zip with `powershell -File scripts/package.ps1`. The app and zip
are written under `dist/`, which is ignored. GitHub Actions also checks Windows
and Linux builds on pushes and pull requests.

`beta.md` and `freemium.md` are local planning documents, removed from current Git
tracking but retained locally. They may still exist in older Git history.
`test/`, `tests/`, `target/`, and `dist/` are ignored. Reproducible Rust unit tests
live alongside source so they remain available to contributors and CI.

Read [architecture and notation](docs/architecture.md),
[beta validation](docs/beta-validation.md), and [research](research/README.md).

MIT licensed. **Generate possibilities, not songs.**
