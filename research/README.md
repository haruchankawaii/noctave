# Research, without invented evidence

All shipped profiles are **experimental**. Their confidence is 0, sample track
count is 0, and sample artist count is 0. We have not annotated real recordings
or run musician blind tests. The Japan / 2000s options are provisional
sound-design modifiers, not established regional or historical findings.

## A reproducible first pipeline

1. Define a style question, region, era, related styles, and inclusion criteria.
2. Select several artists, releases, and tracks. Cap one artist's contribution.
3. Record each source and its actual usage permission before importing data.
4. Have a musician annotate key, mode, relative harmony, extensions, inversions,
   section boundaries, and uncertainty. Use a second reviewer for ambiguous cases.
5. Normalize Roman symbols with the documented Noctave dialect, which measures
   degrees against major (natural minor: i, ii°, bIII, iv, v, bVI, bVII).
6. Run the analyzer. It validates notes and symbols and counts vocabulary,
   transitions, section endings, lengths, and distinct artist/track samples.
7. Compare the report with counterexamples and a held-out artist set. Change
   profile weights deliberately; never import observed frequencies automatically.
8. Compare generic Post-Rock and proposed regional variants in a blind listening
   session with musicians. Record the assessment protocol and disagreement.

Run the included **synthetic**, original example (it is not research evidence):

```sh
cargo run -p harmony-engine --example analyze_corpus -- research/example-corpus.json
```

The report is printed as JSON. For a real study, copy the example outside this
repository, replace the original synthetic phrases with legally usable
annotations, and set `synthetic: false`. The analyzer requires a declared license,
usage category, and `license_verified: true`, but cannot verify legal permissions
for you. Unknown/REFERENCE_ONLY sources are rejected. It never fetches data.

## References

- [Open Music Theory: triads and seventh chords](https://openmusictheory.github.io/triads.html)
  is the theory cross-check for triadic construction and chord qualities.
- [music21 Roman numeral documentation](https://music21.org/music21docs/moduleReference/moduleRoman.html)
  documents spelling, inversions, borrowed harmony, and alternate minor conventions.
  Noctave's explicit major-relative degree convention differs from music21's
  default context-sensitive VI/VII interpretation in minor.
- [Isophonics](http://isophonics.net/) and [McGill Billboard](https://ddmal.music.mcgill.ca/research/The_McGill_Billboard_Project_(Chord_Analysis_Dataset)/)
  are candidates for later dataset investigation, not imported dependencies.
- [MusicBrainz](https://musicbrainz.org/) is a potential metadata/taxonomy reference.
  [Hooktheory](https://www.hooktheory.com/trends) is a potential exploration reference.
  No data from either service is bundled or used by the generator.

Source availability and permissions must be checked for the exact dataset version
before use. The repository contains no recordings, lyrics, tabs, commercial
chord databases, or third-party song transcriptions.

## Research log template

For a study, record: research question; corpus inclusion/exclusion; artists and
tracks; source links and permissions; annotation reviewers; normalization rules;
observations; counterexamples; uncertainties; held-out comparison; listening-test
results; proposed profile changes. A few examples do not validate a style.

## Listening checklist

- Does the progression feel musical and give you an idea worth pursuing?
- Can you distinguish styles without seeing their labels?
- Does changing creativity expand the vocabulary without losing coherence?
- Does lock/regenerate make a convincing continuation?
- Are tension targets and explanations useful, even when you disagree?

Record the full JSON sketch, engine version, seed, and request alongside feedback.
