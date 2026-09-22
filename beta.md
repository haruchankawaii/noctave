# Harmonic Engine — Beta

> Procedural chord progression generator berbasis music theory, probabilistic decision-making, dan style profiling.  
> **No AI. No cloud generation. No black box.**

## Status

**Development Stage:** Beta / Research Prototype  
**Primary Language:** Rust  
**Distribution:** Free Beta  
**Processing:** 100% local  
**AI/ML Runtime:** None

---

# 1. Overview

Harmonic Engine adalah software generatif untuk membantu musisi menemukan chord progression berdasarkan preferensi musikal yang spesifik.

Tujuan proyek ini bukan sekadar menghasilkan progression seperti:

```text
I → V → vi → IV
```

tetapi membangun sistem yang memahami berbagai karakteristik harmoni seperti:

```text
Genre
Subgenre
Regional style
Era
Mood
Complexity
Tension
Chord vocabulary
Modal behavior
Cadence
Voice leading
Bass movement
Repetition
Harmonic movement
```

Contoh target penggunaan:

```text
Genre        : Rock
Subgenre     : Post-Rock
Region       : Japan
Era          : 2000s
Mood         : Melancholic
Energy       : Slow Build
Complexity   : Medium
Experimental : 35%
Key          : D Major
Length       : 8 chords
```

Contoh output:

```text
Dmaj9
F#m7
Gmaj7
Bm7
Em9
Gm6
D/A
Asus4
```

Setiap progression dihasilkan melalui kombinasi:

```text
Music Theory
+
Style Profile
+
Constraints
+
Weighted Randomness
+
Progression Scoring
+
Voice Leading
```

Bukan melalui model generative AI.

---

# 2. Philosophy

Harmonic Engine dibangun berdasarkan beberapa prinsip.

## 2.1 Procedural, bukan preset

Progression tidak dipilih dari daftar progression yang sudah dibuat sebelumnya.

Hindari:

```rust
let progressions = [
    ["I", "V", "vi", "IV"],
    ["vi", "IV", "I", "V"],
];
```

Sebaliknya, progression dibangun melalui hubungan antar chord.

```text
Current Chord
      ↓
Generate Candidates
      ↓
Apply Constraints
      ↓
Style Scoring
      ↓
Theory Scoring
      ↓
Weighted Selection
      ↓
Next Chord
```

---

# 3. Goals

Beta bertujuan membuktikan lima hal.

1. Progression rule-based dapat terasa musikal.
2. Style profile dapat menghasilkan karakter yang berbeda.
3. Regional style dapat dimodelkan tanpa AI.
4. Hasil dapat memiliki variasi sangat besar tanpa menjadi random garbage.
5. Musisi dapat mengontrol hasil lebih dalam dibanding generator chord sederhana.

---

# 4. Non-Goals Beta

Beta belum menargetkan:

- DAW plugin / VST.
- Automatic song composition.
- Melody generation.
- Audio generation.
- Stem generation.
- AI recommendation.
- User account cloud.
- Cloud project sync.
- Marketplace preset.
- Automatic genre recognition dari audio.
- Production-ready commercial licensing.

Fokus beta adalah **harmonic generation engine**.

---

# 5. Core Architecture

```text
User Preferences
       │
       ▼
Style Resolver
       │
       ▼
Harmonic Profile
       │
       ▼
Constraint Builder
       │
       ▼
Candidate Generator
       │
       ▼
Progression Search
       │
       ├── Theory Score
       ├── Style Score
       ├── Tension Score
       ├── Voice-Leading Score
       ├── Novelty Score
       └── Repetition Score
       │
       ▼
Weighted Selection
       │
       ▼
Mutation / Refinement
       │
       ▼
Voicing Generator
       │
       ▼
Final Progression
```

---

# 6. Suggested Rust Architecture

```text
harmonic-engine/
│
├── Cargo.toml
│
├── crates/
│   │
│   ├── harmony-core/
│   │   └── src/
│   │       ├── note.rs
│   │       ├── interval.rs
│   │       ├── pitch_class.rs
│   │       ├── scale.rs
│   │       ├── chord.rs
│   │       ├── chord_quality.rs
│   │       └── key.rs
│   │
│   ├── harmony-theory/
│   │   └── src/
│   │       ├── degree.rs
│   │       ├── function.rs
│   │       ├── cadence.rs
│   │       ├── modal_interchange.rs
│   │       ├── secondary_dominant.rs
│   │       └── substitution.rs
│   │
│   ├── harmony-style/
│   │   └── src/
│   │       ├── profile.rs
│   │       ├── resolver.rs
│   │       ├── inheritance.rs
│   │       └── modifier.rs
│   │
│   ├── harmony-generator/
│   │   └── src/
│   │       ├── request.rs
│   │       ├── candidate.rs
│   │       ├── graph.rs
│   │       ├── transition.rs
│   │       ├── generator.rs
│   │       ├── mutation.rs
│   │       └── seeded_rng.rs
│   │
│   ├── harmony-scoring/
│   │   └── src/
│   │       ├── theory.rs
│   │       ├── style.rs
│   │       ├── tension.rs
│   │       ├── novelty.rs
│   │       └── voice_leading.rs
│   │
│   └── harmony-voicing/
│       └── src/
│           ├── voicing.rs
│           ├── inversion.rs
│           └── optimizer.rs
│
├── data/
│   ├── taxonomy/
│   ├── styles/
│   ├── theory/
│   └── research/
│
├── research/
│   ├── corpus/
│   ├── annotations/
│   ├── reports/
│   └── methodology/
│
└── tests/
```

---

# 7. Fundamental Music Types

Contoh model awal:

```rust
pub enum Degree {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
}
```

```rust
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Augmented,

    Major7,
    Minor7,
    Dominant7,
    HalfDiminished7,
    Diminished7,

    Add9,
    MinorAdd9,
    Sus2,
    Sus4,
}
```

Tetapi implementasi final sebaiknya tidak mengandalkan enum untuk seluruh kemungkinan chord extension.

Gunakan struktur composable seperti:

```rust
Chord {
    root,
    third,
    fifth,
    seventh,
    extensions,
    alterations,
    bass,
}
```

agar engine dapat menangani:

```text
C
Cm
C7
Cmaj7
Cm7
C9
Cmaj9
Cm11
C13
Cadd9
Csus2
Csus4
Cmaj7#11
C7b9
C/E
```

tanpa membuat enum terus bertambah.

---

# 8. Roman-Numeral Representation

Internal progression sebaiknya disimpan relatif terhadap key.

Contoh:

```text
Imaj9
iii7
IVmaj7
vi7
ii9
iv6
I6/4
Vsus4
```

Bukan:

```text
Dmaj9
F#m7
Gmaj7
Bm7
Em9
Gm6
D/A
Asus4
```

Render ke absolute chord dilakukan terakhir.

Keuntungannya:

```text
I → IV → V
```

bisa digunakan pada:

```text
C Major
C → F → G
```

atau:

```text
E Major
E → A → B
```

tanpa menyimpan progression terpisah.

---

# 9. Style System

Style bukan kumpulan progression.

Style adalah kumpulan distribusi dan modifier.

Contoh:

```toml
id = "post-rock"
name = "Post-Rock"

[harmony]
diatonic = 0.70
modal = 0.65
chromatic = 0.20
modal_interchange = 0.45

[chord_quality]
major = 0.75
minor = 0.75
major7 = 0.50
minor7 = 0.55
add9 = 0.65
sus2 = 0.55
sus4 = 0.50

[movement]
stepwise_bass = 0.70
common_tone = 0.75
root_fifth = 0.45

[cadence]
authentic = 0.30
plagal = 0.65
deceptive = 0.50
unresolved = 0.70

[structure]
repetition = 0.70
long_form = 0.75
```

Nilai di atas hanyalah contoh format, bukan hasil penelitian.

---

# 10. Style Inheritance

Style profile menggunakan inheritance.

```text
Rock
 ↓
Post-Rock
 ↓
Japanese Post-Rock
 ↓
2000s Japanese Post-Rock
 ↓
User Preferences
```

Contoh:

```toml
id = "japanese-post-rock"

inherits = [
    "rock",
    "post-rock"
]
```

Regional profile hanya menyimpan perbedaannya.

```toml
[chord_quality]
add9_modifier = 1.20
major7_modifier = 1.15

[movement]
common_tone_modifier = 1.10
```

Angka sebenarnya harus berasal dari research.

---

# 11. Genre Taxonomy

Taxonomy jangan dianggap sebagai kebenaran musik absolut.

Gunakan:

```text
Root Genre
Genre
Subgenre
Microgenre
Scene
Regional Style
Era Style
Trait
```

Contoh:

```text
Rock
└── Post-Rock
    ├── Regional Style: Japan
    │   ├── Era: 1990s
    │   ├── Era: 2000s
    │   └── Era: 2010s
    │
    └── Traits
        ├── Atmospheric
        ├── Cinematic
        ├── Math-influenced
        └── Heavy
```

Beberapa style harus dapat memiliki multiple parent atau relationship.

Contoh:

```text
Math Rock
 ↔ Post-Rock

Shoegaze
 ↔ Dream Pop
 ↔ Noise Rock
```

Gunakan graph bila hierarchy tree tidak cukup.

---

# 12. Beta Genre Scope

Jangan mencoba mengimplementasikan semua genre pada beta pertama.

## Root Genre Candidates

```text
Rock
Pop
Metal
Jazz
R&B / Soul
Electronic
Folk
Classical
Hip-Hop
```

## Initial Research Cluster

Beta pertama fokus pada:

```text
Rock
├── Post-Rock
├── Math Rock
├── Shoegaze
├── Indie Rock
└── Emo / Midwest Emo
```

Alasannya: kelompok ini menguji banyak kebutuhan engine sekaligus:

- modal harmony
- extended chords
- add chords
- suspended chords
- modal interchange
- non-functional movement
- common-tone movement
- bass movement
- repetition
- irregular progression length
- chromatic movement
- open voicing
- tension/release

---

# 13. First Reference Style

Style pertama yang dijadikan end-to-end research case:

```text
Rock
└── Post-Rock
    └── Japanese Post-Rock
```

Tujuannya bukan membuat:

```text
"Generate like Artist X"
```

tetapi memperoleh profile agregat seperti:

```text
Japanese Post-Rock
+
Era
+
Mood
+
Harmonic Traits
```

---

# 14. Research Methodology

Setiap style harus melalui research sebelum diberi status production-ready.

## Step 1 — Define Style

Tentukan:

```text
name
parent
related styles
region
era
known characteristics
research scope
```

---

## Step 2 — Select Corpus

Jangan menggunakan satu artis sebagai representasi seluruh style.

Corpus ideal terdiri dari:

```text
multiple artists
multiple releases
multiple eras
multiple tracks
```

Hindari dominasi satu artis.

---

## Step 3 — Annotate Tracks

Minimal catat:

### Harmony

```text
Key
Mode
Roman numeral
Chord quality
Extension
Inversion
Borrowed chord
Secondary dominant
Chromatic chord
Cadence
```

### Movement

```text
Root movement
Bass movement
Common tones
Voice-leading distance
```

### Structure

```text
Progression length
Section
Repetition
Variation frequency
Climax position
```

### Optional

```text
Tempo
Meter
Dynamics
Texture
Arpeggio behavior
Open-string usage
```

---

# 15. Research Data Format

Contoh:

```toml
track_id = "research_001"
style = "japanese-post-rock"

[[sections]]
name = "verse"
key = "D"
mode = "major"

progression = [
    "Imaj7",
    "iii7",
    "IVadd9",
    "vi7"
]
```

Untuk data publik atau komersial, jangan menyimpan metadata/audio yang lisensinya tidak memungkinkan.

---

# 16. Normalize Before Analysis

Progression:

```text
Dmaj7 → F#m7 → Gadd9 → Bm7
```

dinormalisasi menjadi:

```text
Imaj7 → iii7 → IVadd9 → vi7
```

Sehingga data bisa dibandingkan lintas key.

---

# 17. Transition Matrix

Engine dapat membangun matriks:

```text
FROM → TO

I → ii
I → iii
I → IV
I → V
I → vi

ii → V
ii → IV
...
```

Setiap edge menyimpan:

```rust
Transition {
    target,
    base_weight,
    style_weight,
    tension_delta,
    confidence,
}
```

---

# 18. Weighted Randomness

Jangan selalu memilih chord dengan score tertinggi.

Contoh:

```text
Bm7      0.92
Em9      0.84
Asus4    0.77
Gm6      0.63
Cmaj7    0.45
```

ubah menjadi probability distribution.

Tujuannya:

```text
musically constrained
+
non-deterministic
```

---

# 19. Creativity Control

User dapat memilih:

```text
Conservative ───────── Wild
```

Parameter ini memengaruhi distribusi pilihan.

Semakin conservative:

```text
high-score candidates dominate
```

Semakin wild:

```text
lower-score valid candidates
memiliki kemungkinan lebih tinggi
```

Wild tidak berarti teori diabaikan.

---

# 20. Seeded Generation

Semua generation harus dapat direproduksi.

Contoh:

```text
Seed: 29481932
```

Seed sama + parameter sama = progression sama.

Gunakan seeded PRNG pada Rust.

---

# 21. Candidate Search

Generator tidak perlu hanya membangun satu progression.

Contoh:

```text
Generate 1,000–10,000 candidates
       ↓
Score candidates
       ↓
Reject invalid
       ↓
Select high-quality pool
       ↓
Weighted random final selection
```

Score:

```text
final_score =
    theory_score
  + style_score
  + tension_score
  + voice_leading_score
  + novelty_score
  + structural_score
```

Bobot score menjadi configurable.

---

# 22. Tension Curve

Progression dapat mengikuti target:

```text
Low
 ↓
Medium
 ↓
High
 ↓
Climax
 ↓
Release
```

Setiap chord diberi context-dependent tension score.

Jangan menganggap satu chord memiliki tension universal.

Tension bergantung pada:

```text
key
previous chord
next chord
bass
voicing
style
context
```

---

# 23. Lock & Regenerate

User dapat mengunci bagian yang disukai.

```text
Dmaj9    🔒
F#m7     🔒
Gmaj7
Bm7
Em9      🔒
Gm6
D/A
Asus4    🔒
```

`Regenerate Unlocked` hanya mengganti:

```text
3
4
6
7
```

tetapi tetap mempertimbangkan chord yang terkunci.

---

# 24. Explainability

Setiap chord dapat memiliki explanation:

```text
Gm6
```

```text
Function:
Borrowed iv chord.

Reason:
- raises harmonic tension
- creates minor-color contrast
- preserves common tones
- fits selected style profile

Notes:
G Bb D E
```

Explanation berasal dari rule yang benar-benar digunakan generator.

Jangan generate explanation palsu.

---

# 25. Beta UI

Minimal UI:

```text
Genre
Style
Region
Era

Key
Mode

Mood
Complexity
Creativity

Progression Length

[ Generate ]
```

Result:

```text
Dmaj9
F#m7
Gmaj7
Bm7
Em9
Gm6
D/A
Asus4
```

Controls:

```text
Regenerate
Lock
Unlock
Change Seed
Transpose
Explain
```

---

# 26. Beta Access Model

Beta bersifat gratis.

Untuk mendapatkan feedback yang berguna, style yang sudah siap research boleh dibuka tanpa pembatasan Free/Pro.

Tujuan beta:

```text
validate product
validate generator
validate taxonomy
validate research
```

Bukan monetisasi.

Commercial entitlement dibuat sebagai layer terpisah dan tidak boleh mencemari core engine.

---

# 27. Beta Roadmap

## Phase 0 — Specification

- [ ] Define internal chord representation
- [ ] Define interval representation
- [ ] Define scale representation
- [ ] Define Roman numeral format
- [ ] Define style profile schema
- [ ] Define taxonomy schema
- [ ] Define research schema

---

## Phase 1 — Theory Core

- [ ] Notes
- [ ] Pitch classes
- [ ] Intervals
- [ ] Major scales
- [ ] Minor scales
- [ ] Modes
- [ ] Triads
- [ ] Seventh chords
- [ ] Extensions
- [ ] Inversions
- [ ] Slash chords
- [ ] Roman numerals
- [ ] Transposition

Acceptance test:

```text
Roman numeral + key
→ correct absolute chord
```

---

## Phase 2 — Basic Generator

- [ ] Generate diatonic candidates
- [ ] Weighted transition
- [ ] Seeded RNG
- [ ] Progression length
- [ ] Basic cadence
- [ ] Theory validation

Acceptance test:

```text
same seed
+
same request
=
same progression
```

---

## Phase 3 — Scoring

Implement:

- [ ] theory score
- [ ] transition score
- [ ] repetition score
- [ ] novelty score
- [ ] tension score
- [ ] voice-leading score

---

## Phase 4 — Advanced Harmony

- [ ] modal interchange
- [ ] secondary dominants
- [ ] chromatic mediants
- [ ] pedal harmony
- [ ] common-tone transitions
- [ ] suspended harmony
- [ ] non-functional movement

---

## Phase 5 — Style Engine

- [ ] style loader
- [ ] inheritance
- [ ] style modifiers
- [ ] region modifier
- [ ] era modifier
- [ ] trait modifier
- [ ] profile merging

---

## Phase 6 — Research Pipeline

- [ ] corpus methodology
- [ ] annotation template
- [ ] normalization scripts
- [ ] transition statistics
- [ ] chord vocabulary statistics
- [ ] cadence statistics
- [ ] quality confidence system

---

## Phase 7 — First Deep Style

Research:

```text
Post-Rock
→ Japanese Post-Rock
```

- [ ] select corpus
- [ ] analyze tracks
- [ ] normalize harmony
- [ ] build aggregate profile
- [ ] compare against generic Post-Rock
- [ ] musician blind test

---

## Phase 8 — UI

- [ ] desktop shell
- [ ] generator screen
- [ ] progression cards
- [ ] lock/unlock
- [ ] explanation view
- [ ] settings
- [ ] seed display

---

## Phase 9 — Export

- [ ] copy chord names
- [ ] copy Roman numerals
- [ ] text export
- [ ] JSON export
- [ ] MIDI prototype

---

## Phase 10 — Beta Validation

Collect feedback for:

```text
Does it sound musical?
Does selected style feel different?
Are results repetitive?
Does creativity control work?
Are explanations useful?
Does Lock/Regenerate help composition?
```

---

# 28. Testing Strategy

## Unit Tests

Theory must be heavily tested.

Examples:

```text
C Major:
I    = C
ii   = Dm
iii  = Em
IV   = F
V    = G
vi   = Am
vii° = Bdim
```

Test:

```text
interval construction
scale construction
chord spelling
Roman numeral parsing
transposition
inversion
extensions
```

---

## Property Tests

Contoh:

```text
transpose progression +12 semitones
=
same pitch classes
```

dan:

```text
same seed + same request
=
same output
```

---

## Musical Regression Tests

Simpan seed penting:

```text
seed_001
seed_002
seed_003
```

agar perubahan algoritma dapat dibandingkan.

---

# 29. Research Confidence

Setiap style profile memiliki confidence.

```text
Experimental
Low
Medium
High
Validated
```

Contoh:

```toml
research_status = "experimental"
confidence = 0.42
sample_tracks = 18
sample_artists = 5
```

Jangan menyembunyikan ketidakpastian data.

---

# 30. Required Research Resources

## Music Theory

### Open Music Theory

Gunakan untuk:

- intervals
- triads
- seventh chords
- Roman numerals
- harmonic function
- cadence
- modal mixture
- applied chords
- pop/rock harmony

Resource:

```text
Open Music Theory
openmusictheory.github.io
```

---

### music21 Documentation

Walaupun library-nya Python, dokumentasinya berguna sebagai cross-check implementasi untuk:

```text
Roman numeral interpretation
Chord representation
Inversions
Scale degree
Chord quality
```

Resource:

```text
music21 Documentation
music21.org
```

Tidak perlu menjadi dependency runtime.

---

## Real-World Chord References

### Isophonics Reference Annotations

Berguna untuk mempelajari format anotasi MIR nyata.

Dataset mencakup anotasi seperti:

```text
chords
keys
beats
structural segmentation
```

dan menggunakan Harte chord syntax.

Resource:

```text
Isophonics / Centre for Digital Music
isophonics.net
```

Gunakan terutama sebagai:

```text
research reference
validation reference
annotation-format reference
```

Lisensi setiap dataset harus diperiksa sebelum digunakan atau didistribusikan.

---

### McGill Billboard Dataset

Sangat berguna untuk analisis statistik harmony pada kumpulan lagu.

Potential uses:

```text
chord frequency
transition frequency
section analysis
harmonic vocabulary
Markov-style transition research
```

Resource dapat ditemukan melalui:

```text
McGill Billboard Dataset
mirdata
CoCoPops Billboard corpus
```

Lisensi versi dataset yang digunakan wajib diverifikasi dan dicatat dalam repository.

---

### Hooktheory Trends

Berguna untuk eksplorasi:

```text
next-chord probability
progression frequency
real-song examples
```

Resource:

```text
Hooktheory Trends
hooktheory.com
```

Hooktheory menyediakan API untuk chord probability.

Gunakan sebagai referensi penelitian; jangan menganggap database mereka sebagai dataset bebas untuk dibundel ke produk.

Review terms/API permissions sebelum otomatis mengambil data.

---

## Genre / Artist Research

### MusicBrainz

Gunakan untuk:

```text
artist metadata
release metadata
genre discovery
style terminology
taxonomy research
```

Resource:

```text
MusicBrainz
musicbrainz.org
```

Penting:

Genre bersifat subjektif.

Jangan menganggap MusicBrainz taxonomy sebagai kebenaran absolut.

Perhatikan bahwa bagian berbeda dari data MusicBrainz dapat memiliki lisensi berbeda.

Jangan memasukkan genre/tag dataset ke produk komersial tanpa memeriksa lisensi dataset yang digunakan.

---

# 31. Research Resource Policy

Setiap resource masuk ke salah satu kategori:

```text
REFERENCE_ONLY
RESEARCH_ALLOWED
DATA_IMPORT_ALLOWED
COMMERCIAL_EMBED_ALLOWED
```

Contoh metadata internal:

```toml
name = "example-resource"
usage = "REFERENCE_ONLY"
license_verified = false
```

Jangan import data apabila:

```text
license unclear
terms unclear
commercial use prohibited
redistribution prohibited
```

---

# 32. Copyright / Dataset Safety

Repository tidak boleh membundel tanpa izin:

- copyrighted recordings
- lyrics
- sheet music
- tabs
- proprietary databases
- commercial chord databases
- copyrighted artwork

Lebih aman menyimpan hasil penelitian yang telah diabstraksikan menjadi:

```text
transition probabilities
style weights
statistical summaries
generic harmonic rules
```

daripada mendistribusikan materi sumber.

---

# 33. Research Log

Setiap style memiliki:

```text
research/<style>/README.md
```

Isi:

```text
Research question
Corpus definition
Artists represented
Tracks represented
Sources
Methodology
Normalization rules
Observed patterns
Counterexamples
Confidence
Known limitations
Profile changes
```

---

# 34. Definition of Beta Success

Beta dianggap berhasil jika:

- progression secara umum dianggap musikal
- repeated generation memberikan variasi besar
- seed bekerja deterministically
- subgenre profile terdengar berbeda dari root genre
- regional profile memberi perubahan terukur
- user dapat memahami alasan pemilihan chord
- lock/regenerate menghasilkan continuation yang masuk akal
- generator tetap responsif di device biasa
- tidak membutuhkan internet untuk generation

---

# 35. Guiding Principle

> Generate possibilities, not songs.

Harmonic Engine tidak mencoba menggantikan musisi.

Software bertugas menjelajahi harmonic search space dengan cepat, sementara keputusan kreatif terakhir tetap berada pada musisi.