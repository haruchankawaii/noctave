# Harmonic Engine

> Deep procedural harmony exploration for musicians.
> **No generative AI. Your music stays local.**

## Product Status

**Release Model:** Freemium
**Core Engine:** Rust
**Generation:** Local / Offline
**AI Processing:** None
**Cloud Requirement:** None for generation
**Internet:** Only where required for licensing/update-related services

---

# 1. What Is Harmonic Engine?

Harmonic Engine adalah procedural chord progression generator yang membantu musisi menjelajahi harmony berdasarkan genre, subgenre, regional style, era, mood, complexity, tension, dan harmonic behavior.

Software tidak menggunakan generative AI.

Progression dibangun menggunakan:

```text
Music-theory constraints
Style profiles
Probabilistic transitions
Constraint solving
Candidate search
Progression scoring
Voice-leading optimization
Seeded randomness
```

Hasilnya dapat memiliki ruang kemungkinan yang sangat besar tanpa kehilangan kontrol musikal.

---

# 2. Why?

Generator chord tradisional biasanya berhenti di:

```text
I – V – vi – IV
```

atau:

```text
Randomize
```

Tetapi seorang musisi mungkin membutuhkan:

```text
Japanese Post-Rock
2000s
Melancholic
Moderately Experimental
Slow Build
Open Harmony
8 Chords
Unresolved Ending
```

Harmonic Engine dirancang untuk menjawab kebutuhan tersebut.

---

# 3. Free vs Pro Philosophy

Pembatasan produk dilakukan berdasarkan **depth of style**, bukan kualitas generator.

```text
FREE
Broad genre exploration

PRO
Deep stylistic exploration
```

Free tidak sengaja dibuat buruk.

Free dan Pro menggunakan harmonic engine yang sama.

---

# 4. Free Edition

Free Edition menyediakan **root genres**.

Contoh:

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

User tetap mendapatkan:

- full-quality procedural generation
- key selection
- major/minor selection
- basic modes
- mood
- complexity
- creativity
- progression length
- tension preference
- seed
- regenerate
- transpose
- theory explanation

Root genre memakai broad style profile.

Contoh:

```text
Rock
```

bukan:

```text
Post-Rock
Shoegaze
Math Rock
Progressive Rock
```

---

# 5. Pro Edition

Pro membuka seluruh deep taxonomy.

Contoh:

```text
Rock
└── Post-Rock
    └── Japanese Post-Rock
        └── 2000s
```

Termasuk:

- subgenres
- microgenres
- regional styles
- scene profiles
- era profiles
- deep harmonic controls
- style blending
- advanced voicing
- section-based generation
- advanced tension shaping
- lock & regenerate
- custom style presets
- extended export options

---

# 6. Feature Matrix

| Feature                            | Free | Pro |
| ---------------------------------- | ---: | --: |
| Procedural generator               |    ✓ |   ✓ |
| Root genres                        |    ✓ |   ✓ |
| Key / mode                         |    ✓ |   ✓ |
| Mood                               |    ✓ |   ✓ |
| Complexity                         |    ✓ |   ✓ |
| Creativity                         |    ✓ |   ✓ |
| Seed                               |    ✓ |   ✓ |
| Regeneration                       |    ✓ |   ✓ |
| Transpose                          |    ✓ |   ✓ |
| Chord explanation                  |    ✓ |   ✓ |
| Subgenres                          |   🔒 |   ✓ |
| Microgenres                        |   🔒 |   ✓ |
| Regional styles                    |   🔒 |   ✓ |
| Era-specific profiles              |   🔒 |   ✓ |
| Style blending                     |   🔒 |   ✓ |
| Advanced chord vocabulary controls |   🔒 |   ✓ |
| Advanced tension curve             |   🔒 |   ✓ |
| Lock individual chords             |   🔒 |   ✓ |
| Partial regeneration               |   🔒 |   ✓ |
| Advanced voicing                   |   🔒 |   ✓ |
| MIDI export                        |  TBD |   ✓ |
| Custom profiles                    |   🔒 |   ✓ |

Feature matrix dapat berubah berdasarkan hasil beta.

---

# 7. Example

## Free

```text
Genre:
Rock

Mood:
Melancholic

Complexity:
Medium
```

Output:

```text
D
Bm
G
Em
A
```

Tetap valid dan musikal.

---

## Pro

```text
Genre:
Rock

Subgenre:
Post-Rock

Regional Style:
Japan

Era:
2000s

Mood:
Melancholic

Traits:
Atmospheric
Slow Build
Moderately Experimental
```

Output dapat mengeksplorasi vocabulary dan transition behavior yang jauh lebih spesifik.

---

# 8. Genre Taxonomy

Internal taxonomy:

```text
Root Genre
   ↓
Genre
   ↓
Subgenre
   ↓
Microgenre
   ↓
Regional / Scene Profile
   ↓
Era
```

Traits berada di samping hierarchy:

```text
Atmospheric
Dreamy
Dark
Bright
Cinematic
Aggressive
Minimal
Dense
Experimental
```

---

# 9. Taxonomy Is Not Absolute

Genre tidak selalu membentuk tree yang bersih.

Contoh:

```text
Post-Rock
   ↔ Math Rock
   ↔ Shoegaze
   ↔ Ambient
```

Karena itu internal model mendukung:

```text
parent
related_to
influenced_by
overlaps_with
regional_variant
era_variant
```

---

# 10. Style Profiles

Style bukan preset chord progression.

```toml
id = "example-style"

[harmony]
diatonic = 0.70
modal = 0.55
chromatic = 0.25

[chord_quality]
major7 = 0.60
minor7 = 0.70
add9 = 0.65
sus2 = 0.45

[movement]
common_tone = 0.70
stepwise_bass = 0.60

[cadence]
authentic = 0.40
plagal = 0.65
unresolved = 0.60
```

Nilai profile harus berasal dari research dan validation.

---

# 11. Profile Inheritance

Contoh:

```text
Rock
   ↓
Post-Rock
   ↓
Japanese Post-Rock
   ↓
2000s
   ↓
User Modifiers
```

Final profile:

```text
base profile
×
subgenre modifiers
×
regional modifiers
×
era modifiers
×
user preference modifiers
```

---

# 12. Generation Engine

```text
Request
  ↓
Style Resolution
  ↓
Constraint Construction
  ↓
Candidate Generation
  ↓
Candidate Scoring
  ↓
Search / Optimization
  ↓
Weighted Selection
  ↓
Voicing
  ↓
Explanation
```

---

# 13. Generation Strategies

Engine dapat mencampur beberapa strategy.

```text
Functional Harmony
Modal Harmony
Voice-Leading Driven
Common-Tone Harmony
Pedal Harmony
Modal Interchange
Chromatic Mediant
Bass-Driven Harmony
Suspended Harmony
Repetitive / Ostinato Harmony
```

Bobot strategy berubah berdasarkan style.

---

# 14. Deterministic Seeds

Setiap result memiliki seed.

```text
Seed #29481932
```

Seed + configuration yang sama menghasilkan progression yang sama.

Benefits:

```text
reproducibility
sharing
debugging
preset creation
regression testing
```

---

# 15. Harmonic Creativity

User dapat mengontrol seberapa jauh generator mengeksplorasi kandidat ber-score rendah tetapi masih valid.

```text
Safe ───────────── Experimental
```

Creativity tidak menonaktifkan teori.

Ia mengubah distribution dari valid harmonic choices.

---

# 16. Progression Locking — Pro

User dapat mengunci chord:

```text
Dmaj9   🔒
F#m7    🔒
Gmaj7
Bm7
Em9     🔒
Gm6
D/A
Asus4   🔒
```

Kemudian:

```text
Regenerate Unlocked
```

Engine mencari solusi baru sambil mempertahankan locked context.

---

# 17. Explainability

Semua keputusan penting dapat dijelaskan menggunakan rule yang digunakan engine.

Contoh:

```text
Gm6
```

Reason:

```text
Borrowed iv harmony
Raises tension
Creates color contrast
Maintains common-tone relationship
Matches selected style profile
```

Tidak ada AI-generated explanation.

---

# 18. Privacy

Core generation dijalankan secara lokal.

Tidak perlu meng-upload:

```text
project
chord progression
MIDI
composition
music files
```

untuk menggunakan generator.

---

# 19. Commercial Model

Target desktop:

```text
Free
+
Pro one-time purchase
```

Pro license membuka entitlement fitur.

Generation tetap lokal.

---

# 20. Licensing Architecture

Jangan menggabungkan license system dengan harmony engine.

```text
Harmony Core
      │
      │ does not know
      ▼
Free / Pro
```

Application layer:

```rust
if !entitlements.can_access(style_id) {
    return Err(AccessError::ProRequired);
}
```

Core:

```rust
engine.generate(request)
```

tetap independen.

---

# 21. Entitlement Example

```rust
pub enum ProductTier {
    Free,
    Pro,
}
```

Tetapi style data tidak harus menyimpan entitlement langsung.

Gunakan product manifest:

```toml
[styles]
rock = "free"

post-rock = "pro"
math-rock = "pro"
shoegaze = "pro"
japanese-post-rock = "pro"
```

Sehingga musical dataset tidak bergantung pada monetisasi.

---

# 22. Offline-First Design

Target:

```text
Startup
 ↓
Read local license state
 ↓
Run locally
```

Online operation digunakan hanya untuk kebutuhan seperti:

```text
initial activation
license verification
license recovery
updates
```

sesuai kebijakan produk final.

Generation tidak bergantung pada server.

---

# 23. Research Before Release

Tidak ada deep style yang diberi status stable tanpa research.

Minimum research pipeline:

```text
Define style
 ↓
Build corpus
 ↓
Analyze harmony
 ↓
Normalize to Roman numerals
 ↓
Calculate statistics
 ↓
Build style profile
 ↓
Compare against parent style
 ↓
Musician validation
 ↓
Release
```

---

# 24. Artist Research

Artis digunakan sebagai **evidence untuk pola**, bukan target imitasi.

Jangan membuat positioning seperti:

```text
Generate exactly like Artist X
```

Research sebaiknya menggunakan banyak artis.

Goal:

```text
find shared tendencies
```

bukan:

```text
clone an artist
```

---

# 25. What We Research

Untuk setiap style:

## Chord Vocabulary

```text
triads
7th chords
9th chords
extensions
suspended chords
slash chords
alterations
```

## Harmonic Movement

```text
degree transitions
root motion
bass motion
common tones
chromatic movement
```

## Harmony Type

```text
functional
modal
non-functional
borrowed harmony
secondary dominants
chromatic mediants
pedal harmony
```

## Structure

```text
progression length
repetition
variation
cadence
section behavior
```

---

# 26. Research Confidence

Public-facing styles dapat memiliki internal:

```text
sample_artist_count
sample_track_count
confidence
review_status
last_researched
```

Status:

```text
experimental
reviewed
validated
```

---

# 27. Reference Resources

## Open Music Theory

Primary theoretical reference untuk:

```text
intervals
triads
seventh chords
Roman numerals
harmonic function
cadences
modal mixture
applied harmony
pop/rock harmony
voice leading
```

Resource:

```text
Open Music Theory
openmusictheory.github.io
```

Catatan:

Konten memiliki open license, tetapi derivative content tetap harus mengikuti persyaratan lisensinya.

Lebih aman menggunakan teori musikalnya sebagai referensi implementasi daripada menyalin teksnya.

---

## music21 Documentation

Berguna sebagai implementation reference untuk:

```text
RomanNumeral
Chord
Scale
Inversion
Chord quality
```

Resource:

```text
music21
music21.org
```

Tidak digunakan sebagai runtime dependency.

---

## Isophonics Reference Annotations

Academic MIR resource untuk:

```text
chord annotations
keys
beats
structural segmentation
Harte chord syntax
```

Resource:

```text
Isophonics
isophonics.net
```

Penting:

Verify license masing-masing dataset sebelum memasukkan data ke commercial distribution.

Tidak perlu memasukkan audio asli.

---

## McGill Billboard Dataset

Research corpus yang berguna untuk:

```text
real-song harmony
transition statistics
chord vocabulary
harmonic structure
```

Gunakan versioned copy yang lisensinya sudah diverifikasi.

Catat:

```text
dataset version
source
license
date retrieved
processing performed
```

di repository.

---

## Hooktheory Trends / API

Berguna untuk membandingkan:

```text
next-chord probability
common progressions
progression examples
```

Resource:

```text
Hooktheory
hooktheory.com
```

Jangan scrape atau membundel proprietary database.

Gunakan API hanya sesuai terms yang berlaku.

---

## MusicBrainz

Gunakan untuk research:

```text
artists
releases
recordings
areas
genre terminology
genre discovery
```

Resource:

```text
MusicBrainz
musicbrainz.org
```

Penting:

MusicBrainz memiliki beberapa kategori data dengan lisensi berbeda.

Genre/tag associations tidak boleh diasumsikan otomatis aman untuk commercial embedding.

Verifikasi dataset dan lisensinya terlebih dahulu.

---

# 28. Dataset Manifest

Semua resource eksternal yang pernah diimport harus masuk:

```text
data/SOURCES.toml
```

Contoh:

```toml
[[source]]
name = "example"
version = "1.0"
purpose = "research"
license = "VERIFY"
redistributed = false
commercial_use_verified = false
retrieved = "YYYY-MM-DD"
```

Build production harus dapat menolak dataset:

```text
commercial_use_verified = false
```

jika data tersebut akan dibundel ke aplikasi.

---

# 29. Recommended Commercial Data Strategy

Prioritaskan urutan berikut:

```text
1. Music theory implemented manually
2. Original hand-created rule system
3. Licensed/open annotation datasets
4. Aggregate statistics produced internally
5. Human musician validation
```

Hindari menjadikan proprietary chord database sebagai dependency inti.

Produk harus tetap bisa dibangun dari research yang legal dan reproducible.

---

# 30. Research Output

Dataset research mentah bukan produk.

Yang masuk production idealnya berupa abstract profile:

```toml
[transition.I]
IV = 0.41
vi = 0.27
iii = 0.12

[chord_quality]
major7 = 0.52
minor7 = 0.61
add9 = 0.69

[cadence]
plagal = 0.64
unresolved = 0.58
```

Bukan salinan progression seluruh lagu.

---

# 31. Product Roadmap

## Beta

```text
Theory Core
→ Generator
→ Style System
→ Research Pipeline
→ Japanese Post-Rock Case Study
→ UI
→ User Testing
```

## v1.0

```text
Root Genres
Core Pro Subgenres
Licensing
MIDI Export
Stable UI
Installer
Documentation
```

## v1.x

```text
More Subgenres
Regional Profiles
Era Profiles
Better Voicing
Research Expansion
```

## v2

Candidates:

```text
section-based song harmony
style blending
custom harmonic DNA
advanced guitar voicing
advanced piano voicing
DAW integration
plugin formats
```

---

# 32. v1 Genre Priority

Recommended initial priority:

```text
Rock
├── Post-Rock
├── Math Rock
├── Shoegaze
├── Indie Rock
├── Emo
└── Progressive Rock

Metal
├── Heavy Metal
├── Doom Metal
├── Progressive Metal
└── Post-Metal

Pop
├── Pop
├── Indie Pop
├── Dream Pop
└── City Pop

Jazz
├── Jazz
├── Modal Jazz
├── Fusion
└── Contemporary Jazz

R&B / Soul
├── Soul
├── Neo Soul
├── Funk
└── Contemporary R&B
```

Tambah style berdasarkan kualitas research, bukan jumlah.

---

# 33. Release Quality Gate

Sebelum v1:

- [ ] deterministic seeded generation
- [ ] zero known theory-critical bugs
- [ ] style inheritance stable
- [ ] root genre profiles validated
- [ ] first deep subgenres validated
- [ ] entitlement isolated from engine
- [ ] offline generation confirmed
- [ ] external data license audit completed
- [ ] research source manifest complete
- [ ] regression suite passing
- [ ] installer tested
- [ ] license activation tested
- [ ] beta feedback reviewed

---

# 34. Success Metric

Success bukan:

```text
How many progressions can the software technically generate?
```

Success adalah:

```text
Can a musician repeatedly generate
different progressions that still feel
appropriate for the musical context
they requested?
```

---

# 35. Product Principle

> Free gives you the genre.
> Pro gives you the musical identity.

Harmonic Engine tidak mencoba menggantikan kreativitas musisi.

Ia memperbesar ruang eksplorasi harmoni yang bisa dijelajahi oleh musisi.
