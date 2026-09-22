# Beta validation record

Noctave 0.1.0-beta.1 was developed and exercised on Windows x64. This is a software
validation record, not a claim that musical style or listener preference has been
validated. Freemium development is outside this beta.

## Automated checks

- Rust: theory spelling, all seven mode chord qualities, Roman parsing and
  rendering, inversions, octave transposition, complete MIDI voicings.
- Engine: identical seed/request gives identical full output; partial and full
  locks survive generation; transposition preserves relative structure;
  restoration preserves score/explanations without inventing locks;
  conservative generation stays diatonic; different styles/seeds/traits affect
  output; simple complexity and resolved endings obey their constraints.
- Research: synthetic data remains explicitly marked; section boundaries do not
  create false transitions; duplicate track IDs and unverified sources fail.
- HTTP/MIDI: bad input is rejected, MIDI bytes are constructed from canonical
  chord data, and file headers and track sizes are valid.
- Bun/TypeScript: strict type checking, escaped user text, corrupted browser
  storage, malformed imports, session recovery, and playback timing validation.
- HTTP integration script: embedded UI and bundle, deterministic generation,
  locks, restoration, transposition, MIDI, invalid JSON requests, cross-origin
  rejection, body-size limit, and fixed-path asset serving.

Commands and repeatable checks are in the main README and `scripts/smoke.ts`.
The CI workflow repeats the Rust, Bun, format, build, and HTTP checks on Windows
and Linux. Only local Windows results can be claimed before CI has run.

## Manual browser checks

The studio was opened and inspected at desktop and narrow-screen sizes. Verified:
initial generation; locking positions 1 and 5 and regenerating the other positions;
saving and reopening a named sketch; transposing D to E; starting playback; MIDI
download; no browser console errors in those flows. Responsive layout fits a
390-pixel-wide viewport without horizontal overflow.

These checks are repeated against the TypeScript-built release during packaging.
They do not measure listener judgments, timbral realism, or accessibility with a
screen reader. Browser-specific audio behavior beyond the tested Chromium
environment has not been certified.

## Measured local responsiveness

An integration run measured 12 release-build generations at the largest beta
settings: 16 chords, rich complexity, and 100% creativity. Median HTTP completion
was approximately 236 ms and the slowest was approximately 436 ms on this machine.
These are observations from one run, not a performance guarantee.

## Remaining musical validation

Real corpus annotation, held-out artist comparisons, controlled style-difference
tests, musician blind listening, and regional/era validation remain future work.
All shipped style weights are hypotheses with zero claimed annotated tracks.
