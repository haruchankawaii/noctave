# Noctave beta architecture

Noctave 0.1 is an offline harmonic sketchbook. A Rust executable embeds its web
interface and serves it only on the loopback interface. Windows uses an Edge app
window when available; other systems open the default browser. No account,
external assets, telemetry, cloud generation, or AI runtime is needed.

## Boundaries

- `harmony-core`: spelled notes, modes, relative composable chords, Roman numeral
  rendering and parsing, transposition, and voice allocation.
- `harmony-engine`: seeded search, style inheritance, constraints, score traces,
  locked continuations, tension, and MIDI serialization.
- `noctave`: loopback HTTP transport, embedded assets, CLI, and desktop launcher.
- `ui`: dependency-free HTML, CSS, and strict TypeScript; Web Audio audition, local
  sketchbook storage, and accessible editing controls.

TypeScript is built with Bun. Cargo's build script runs `scripts/build.ts` and
embeds its browser bundle from `OUT_DIR`; generated JavaScript stays inside ignored
build output. There is no authored JavaScript or Node/npm runtime. `bun run dev`
uses a Bun loopback server that compiles the TypeScript on reload and proxies the
Rust engine. Production needs only the compiled Rust executable and a browser.

The same engine powers both the UI and CLI. The browser does not generate chords.
Seed + request + engine/profile version reproduce the same progression. Locked
chords are relative structures supplied as part of that request. Changing key
transposes their notes; changing mode clears locks in the UI.

## Representation

Degree is zero-based internally and measured against the major scale, including
in minor/modal keys (e.g. natural-minor degrees are i, ii°, bIII, iv, v, bVI,
bVII). Accidentals explicitly specify the offset. Chord tones carry diatonic
interval numbers and semitone distances, preserving spelling even for E# or Cb.
Extensions and inversions compose with a triad rather than enumerating every
possible chord. `add6` means an added sixth; `(inv1)`/`(inv2)` identify inversions
in extended Roman symbols to avoid confusing an added sixth with figured bass.
Standard triadic `I6` and `I6/4` are accepted by the parser.

## Search and evidence

Each next-chord candidate has a trace of actual scoring contributions. Style,
harmonic movement, common tones, novelty, repetition, phrase position, and a
contextual tension target influence weighted selection. A pool of full phrases
is searched before selecting a result. Locks influence both incoming and outgoing
transitions. Final voice allocation minimizes motion across possible inversions.

Profiles are hand-authored hypotheses, explicitly `experimental`, confidence 0,
with no claimed source tracks or artist samples. The Japan/era profiles are
sound-design modifiers, not validated statements about those music scenes.
Research must precede stronger claims. See `research/README.md`.

## Early-beta boundaries

One Rock family, seven diatonic modes, 2–16 chords, 4/4 MIDI with a single piano
track, a simple synthesis preview, and local browser storage. No DAW plugin,
melody/audio generation, artist imitation, native installer, automatic corpus
import, or verified genre recognition. Musical quality needs listening feedback.
