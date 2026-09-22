//! Seeded, transparent procedural generation for Noctave.
#[cfg(test)]
mod checks;
pub mod midi;
mod rng;
mod scoring;
pub mod style;
use harmony_core::{voice, Chord, ChordSource, Interval, Mode, Note, MAJOR};
use rng::Rng;
use serde::{Deserialize, Serialize};
use style::Profile;
pub const ENGINE_VERSION: &str = "0.1.0-beta.1";
macro_rules! choice {
    ($name:ident { $($value:ident),+ })=>{
        #[derive(Debug,Clone,Copy,PartialEq,Eq,Serialize,Deserialize)]
        #[serde(rename_all="kebab-case")]
        pub enum $name {$($value),+}
    }
}
choice!(Mood {
    Neutral,
    Melancholic,
    Hopeful,
    Dreamy,
    Restless
});
choice!(Complexity {
    Simple,
    Balanced,
    Rich
});
choice!(Energy {
    Steady,
    SlowBuild,
    Wave,
    Resolve
});
choice!(Cadence {
    Open,
    Resolved,
    Loop
});
choice!(Trait {
    None,
    Atmospheric,
    Cinematic,
    MathInfluenced,
    Heavy
});

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Request {
    pub key: String,
    pub mode: Mode,
    pub style: String,
    pub region: String,
    pub era: String,
    pub mood: Mood,
    pub complexity: Complexity,
    pub creativity: u8,
    pub length: u8,
    pub seed: u32,
    pub energy: Energy,
    pub cadence: Cadence,
    pub harmonic_trait: Trait,
    pub tempo: u16,
    pub beats_per_chord: u8,
    pub locks: Vec<Option<Chord>>,
}
impl Default for Request {
    fn default() -> Self {
        Self {
            key: "D".into(),
            mode: Mode::Major,
            style: "post-rock".into(),
            region: "global".into(),
            era: "any".into(),
            mood: Mood::Dreamy,
            complexity: Complexity::Balanced,
            creativity: 35,
            length: 8,
            seed: 29481932,
            energy: Energy::SlowBuild,
            cadence: Cadence::Open,
            harmonic_trait: Trait::None,
            tempo: 88,
            beats_per_chord: 4,
            locks: vec![],
        }
    }
}
impl Request {
    pub fn validate(&self) -> Result<(Note, Profile), String> {
        let key = Note::parse(&self.key)?;
        if !(2..=16).contains(&self.length) {
            return Err("Choose between 2 and 16 chords".into());
        }
        if self.creativity > 100 {
            return Err("Creativity must be 0–100".into());
        }
        if !(40..=220).contains(&self.tempo) || ![1, 2, 4, 8].contains(&self.beats_per_chord) {
            return Err("Tempo must be 40–220; chord duration must be 1, 2, 4, or 8 beats".into());
        }
        if ![
            "rock",
            "post-rock",
            "math-rock",
            "shoegaze",
            "indie-rock",
            "midwest-emo",
        ]
        .contains(&self.style.as_str())
        {
            return Err("Unknown beta style".into());
        }
        if !["global", "japan"].contains(&self.region.as_str())
            || !["any", "2000s"].contains(&self.era.as_str())
        {
            return Err("Unsupported region or era".into());
        }
        if self.region == "japan" && self.style != "post-rock" {
            return Err("The exploratory Japan profile is available for Post-Rock only".into());
        }
        if self.era != "any" && self.region != "japan" {
            return Err("The exploratory 2000s profile requires Japan / Post-Rock".into());
        }
        if !self.locks.is_empty() && self.locks.len() != self.length as usize {
            return Err("Locks must match progression length".into());
        }
        for chord in self.locks.iter().flatten() {
            chord.validate()?;
        }
        let id = if self.region == "japan" {
            if self.era == "2000s" {
                "post-rock-japan-2000s"
            } else {
                "post-rock-japan"
            }
        } else {
            &self.style
        };
        let mut profile = style::resolve(id)?;
        for (name, delta) in match self.harmonic_trait {
            Trait::None => vec![],
            Trait::Atmospheric => vec![
                ("open_voicing", 0.2),
                ("suspended", 0.15),
                ("resolution", -0.15),
            ],
            Trait::Cinematic => vec![("extensions", 0.15), ("common_tone", 0.1)],
            Trait::MathInfluenced => vec![
                ("extensions", 0.15),
                ("repetition", -0.25),
                ("stepwise", 0.15),
            ],
            Trait::Heavy => vec![("extensions", -0.3), ("fifths", 0.2), ("repetition", 0.2)],
        } {
            let value = (profile.weight(name) + delta).clamp(0.0, 1.0);
            profile.weights.insert(name.into(), value);
        }
        Ok((key, profile))
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub theory: f64,
    pub style: f64,
    pub movement: f64,
    pub tension: f64,
    pub repetition: f64,
    pub phrase: f64,
    pub total: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedChord {
    pub chord: Chord,
    pub name: String,
    pub roman: String,
    pub notes: Vec<String>,
    pub midi: Vec<u8>,
    pub function: String,
    pub tension: f64,
    pub target_tension: f64,
    pub reasons: Vec<String>,
    pub score: Score,
    pub locked: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Progression {
    pub engine_version: String,
    pub request: Request,
    pub profile: Profile,
    pub scale: Vec<String>,
    pub chords: Vec<RenderedChord>,
    pub searched: usize,
    pub score: f64,
}
fn candidates(request: &Request) -> Vec<Chord> {
    let mut pool = vec![];
    for degree in 0..7 {
        let basic = Chord::diatonic(request.mode, degree, false);
        pool.push(basic.clone());
        for third in [2, 5] {
            let mut chord = basic.clone();
            chord.third = third;
            if chord.fifth == 7 && chord.is_diatonic(request.mode) {
                pool.push(chord);
            }
        }
        if request.complexity != Complexity::Simple {
            pool.push(Chord::diatonic(request.mode, degree, true));
            if basic.fifth == 7 {
                let mut chord = basic.clone();
                chord.extensions.push(Interval {
                    degree: 9,
                    semitones: 14,
                });
                if chord.is_diatonic(request.mode) {
                    pool.push(chord);
                }
            }
        }
        if request.complexity == Complexity::Rich && basic.fifth == 7 {
            let mut ninth = Chord::diatonic(request.mode, degree, true);
            ninth.extensions.push(Interval {
                degree: 9,
                semitones: 14,
            });
            if ninth.is_diatonic(request.mode) {
                pool.push(ninth.clone());
            }
            if basic.is_minor() {
                ninth.extensions.push(Interval {
                    degree: 11,
                    semitones: 17,
                });
                if ninth.is_diatonic(request.mode) {
                    pool.push(ninth);
                }
            }
        }
    }
    if request.creativity >= 15 {
        let parallel = if matches!(request.mode, Mode::Major | Mode::Lydian | Mode::Mixolydian) {
            Mode::Minor
        } else {
            Mode::Major
        };
        for degree in [0, 2, 3, 5, 6] {
            let mut chord = Chord::diatonic(parallel, degree, false);
            chord.source = ChordSource::Borrowed;
            if !chord.is_diatonic(request.mode) {
                pool.push(chord.clone());
                if request.complexity != Complexity::Simple && chord.third == 3 && chord.fifth == 7
                {
                    chord.extensions.push(Interval {
                        degree: 6,
                        semitones: 9,
                    });
                    pool.push(chord);
                }
            }
        }
    }
    if request.creativity >= 40 {
        for target in [1usize, 3, 4, 5] {
            let root = (request.mode.intervals()[target] + 7) % 12;
            let degree = (target + 4) % 7;
            let alteration = (root as i16 - MAJOR[degree] as i16 + 6).rem_euclid(12) - 6;
            if (-2..=2).contains(&alteration) {
                pool.push(Chord {
                    degree: degree as u8,
                    alteration: alteration as i8,
                    third: 4,
                    fifth: 7,
                    seventh: Some(10),
                    extensions: vec![],
                    inversion: 0,
                    source: ChordSource::SecondaryDominant,
                });
            }
        }
    }
    pool
}
pub fn generate(request: Request) -> Result<Progression, String> {
    let (key, profile) = request.validate()?;
    let pool = candidates(&request);
    let mut rng = Rng::new(request.seed);
    let temperature = 0.18 + request.creativity as f64 / 100.0 * 1.05;
    let attempts = 96;
    let mut phrases: Vec<(Vec<Chord>, f64)> = Vec::with_capacity(attempts);
    for _ in 0..attempts {
        let mut phrase = Vec::with_capacity(request.length as usize);
        for position in 0..request.length as usize {
            if let Some(Some(locked)) = request.locks.get(position) {
                phrase.push(locked.clone());
                continue;
            }
            let scores: Vec<f64> = pool
                .iter()
                .map(|chord| scoring::evaluate(chord, &phrase, &request, &profile, position).total)
                .collect();
            phrase.push(pool[rng.weighted(&scores, temperature)].clone());
        }
        let score = phrase
            .iter()
            .enumerate()
            .map(|(i, c)| scoring::evaluate(c, &phrase[..i], &request, &profile, i).total)
            .sum::<f64>()
            / phrase.len() as f64;
        phrases.push((phrase, score));
    }
    phrases.sort_by(|a, b| b.1.total_cmp(&a.1));
    let shortlist = &phrases[..16];
    let index = rng.weighted(
        &shortlist.iter().map(|p| p.1).collect::<Vec<_>>(),
        temperature * 0.45,
    );
    render(request, shortlist[index].0.clone(), attempts, key, profile)
}
fn render(
    request: Request,
    mut phrase: Vec<Chord>,
    searched: usize,
    key: Note,
    profile: Profile,
) -> Result<Progression, String> {
    let mut rendered = vec![];
    let mut previous_voice = vec![];
    for i in 0..phrase.len() {
        let locked = request.locks.get(i).is_some_and(Option::is_some);
        let mut chord = phrase[i].clone();
        let open = profile.weight("open_voicing") >= 0.6;
        let mut midi = voice(&chord, key, &previous_voice, open);
        if !locked && !previous_voice.is_empty() && request.complexity != Complexity::Simple {
            let cost = |notes: &[u8], inversion: u8| -> f64 {
                notes
                    .iter()
                    .zip(previous_voice.iter())
                    .enumerate()
                    .map(|(j, (a, b))| a.abs_diff(*b) as f64 * if j == 0 { 1.8 } else { 1.0 })
                    .sum::<f64>()
                    + inversion as f64 * 2.5
            };
            let mut best = cost(&midi, chord.inversion);
            for inversion in 1..=2 {
                let mut alternate = chord.clone();
                alternate.inversion = inversion;
                let notes = voice(&alternate, key, &previous_voice, open);
                let score = cost(&notes, inversion);
                if score < best {
                    best = score;
                    chord = alternate;
                    midi = notes;
                }
            }
        }
        phrase[i] = chord.clone();
        let score = scoring::evaluate(&chord, &phrase[..i], &request, &profile, i);
        let tension = scoring::tension(&chord, phrase.get(i.wrapping_sub(1)), &request, &profile);
        let reasons = scoring::explain(&chord, &phrase[..i], &request, &profile, i, &score, locked);
        rendered.push(RenderedChord {
            name: chord.name(key),
            roman: chord.roman(),
            notes: chord.notes(key).iter().map(|n| n.name()).collect(),
            midi: midi.clone(),
            function: chord.function().into(),
            tension,
            target_tension: scoring::target(&request, i),
            reasons,
            score,
            chord,
            locked,
        });
        previous_voice = midi;
    }
    let score = rendered.iter().map(|c| c.score.total).sum::<f64>() / rendered.len() as f64;
    Ok(Progression {
        engine_version: ENGINE_VERSION.into(),
        scale: request
            .mode
            .intervals()
            .iter()
            .enumerate()
            .map(|(i, n)| key.interval(i as u8 + 1, *n as i16).name())
            .collect(),
        request,
        profile,
        chords: rendered,
        searched,
        score,
    })
}
/// Re-render exact relative chords in a new key without another random search.
pub fn transpose(progression: &Progression, key: &str) -> Result<Progression, String> {
    let mut request = progression.request.clone();
    request.key = key.into();
    let (key, profile) = request.validate()?;
    if progression.chords.len() != request.length as usize {
        return Err("Chord count does not match request".into());
    }
    for c in &progression.chords {
        c.chord.validate()?;
    }
    let mut frozen = request.clone();
    frozen.locks = progression
        .chords
        .iter()
        .map(|c| Some(c.chord.clone()))
        .collect();
    let mut result = render(
        frozen,
        progression.chords.iter().map(|c| c.chord.clone()).collect(),
        progression.searched,
        key,
        profile,
    )?;
    result.request = request;
    for (i, chord) in result.chords.iter_mut().enumerate() {
        chord.locked = result.request.locks.get(i).is_some_and(Option::is_some);
        chord
            .reasons
            .retain(|r| !r.starts_with("This relative chord"));
        if chord.locked {
            chord
                .reasons
                .push("This relative chord and its inversion were preserved by your lock.".into());
        }
    }
    Ok(result)
}
