//! Pure, key-relative music primitives. No filesystem, network, or random state.
use serde::{Deserialize, Serialize};

const LETTERS: [&str; 7] = ["C", "D", "E", "F", "G", "A", "B"];
const NATURAL: [i16; 7] = [0, 2, 4, 5, 7, 9, 11];
pub const MAJOR: [u8; 7] = [0, 2, 4, 5, 7, 9, 11];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Note {
    pub letter: u8,
    pub accidental: i8,
}

impl Note {
    pub fn parse(value: &str) -> Result<Self, String> {
        let mut chars = value.chars();
        let first = chars.next().ok_or("A note cannot be empty")?;
        let letter = LETTERS
            .iter()
            .position(|s| s.starts_with(first.to_ascii_uppercase()))
            .ok_or_else(|| format!("Unknown note: {value}"))? as u8;
        let mut accidental = 0;
        for ch in chars {
            accidental += match ch {
                '#' | '♯' => 1,
                'b' | '♭' => -1,
                _ => return Err(format!("Invalid accidental in {value}")),
            };
            if !(-2..=2).contains(&accidental) {
                return Err("At most two accidentals are supported".into());
            }
        }
        Ok(Self { letter, accidental })
    }

    pub fn pitch_class(self) -> u8 {
        (NATURAL[self.letter as usize] + self.accidental as i16).rem_euclid(12) as u8
    }

    pub fn name(self) -> String {
        format!(
            "{}{}",
            LETTERS[self.letter as usize],
            if self.accidental >= 0 {
                "#".repeat(self.accidental as usize)
            } else {
                "b".repeat((-self.accidental) as usize)
            }
        )
    }

    pub fn interval(self, degree: u8, semitones: i16) -> Self {
        let letter = (self.letter + (degree - 1) % 7) % 7;
        let pitch = (self.pitch_class() as i16 + semitones).rem_euclid(12);
        let accidental = (pitch - NATURAL[letter as usize] + 6).rem_euclid(12) - 6;
        Self {
            letter,
            accidental: accidental as i8,
        }
    }

    pub fn from_pitch(pitch: u8, flats: bool) -> Self {
        let names = if flats {
            [
                "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab", "A", "Bb", "B",
            ]
        } else {
            [
                "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
            ]
        };
        Self::parse(names[(pitch % 12) as usize]).expect("built-in note")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    Major,
    Minor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Locrian,
}

impl Mode {
    pub fn intervals(self) -> [u8; 7] {
        match self {
            Self::Major => MAJOR,
            Self::Minor => [0, 2, 3, 5, 7, 8, 10],
            Self::Dorian => [0, 2, 3, 5, 7, 9, 10],
            Self::Phrygian => [0, 1, 3, 5, 7, 8, 10],
            Self::Lydian => [0, 2, 4, 6, 7, 9, 11],
            Self::Mixolydian => [0, 2, 4, 5, 7, 9, 10],
            Self::Locrian => [0, 1, 3, 5, 6, 8, 10],
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Major => "Major",
            Self::Minor => "Natural minor",
            Self::Dorian => "Dorian",
            Self::Phrygian => "Phrygian",
            Self::Lydian => "Lydian",
            Self::Mixolydian => "Mixolydian",
            Self::Locrian => "Locrian",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Interval {
    pub degree: u8,
    pub semitones: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChordSource {
    Diatonic,
    Borrowed,
    SecondaryDominant,
}

/// Degree is 0..6 relative to major; alteration is explicit in every mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Chord {
    pub degree: u8,
    pub alteration: i8,
    pub third: u8,
    pub fifth: u8,
    pub seventh: Option<u8>,
    pub extensions: Vec<Interval>,
    pub inversion: u8,
    pub source: ChordSource,
}

impl Chord {
    pub fn diatonic(mode: Mode, degree: u8, seventh: bool) -> Self {
        let scale = mode.intervals();
        let index = degree as usize;
        let distance = |steps: usize| (scale[(index + steps) % 7] + 12 - scale[index]) % 12;
        Self {
            degree,
            alteration: scale[index] as i8 - MAJOR[index] as i8,
            third: distance(2),
            fifth: distance(4),
            seventh: seventh.then(|| distance(6)),
            extensions: vec![],
            inversion: 0,
            source: ChordSource::Diatonic,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.degree > 6 || !(-2..=2).contains(&self.alteration) {
            return Err("Chord degree or alteration is out of range".into());
        }
        if ![2, 3, 4, 5].contains(&self.third) || ![6, 7, 8].contains(&self.fifth) {
            return Err("Unsupported chord structure".into());
        }
        if self.seventh.is_some_and(|v| ![9, 10, 11].contains(&v)) {
            return Err("Invalid seventh".into());
        }
        if self.extensions.len() > 3
            || self.extensions.iter().any(|v| {
                !matches!(
                    (v.degree, v.semitones),
                    (6, 9) | (9, 13..=15) | (11, 17..=18) | (13, 20..=21)
                )
            })
        {
            return Err("Unsupported chord extension".into());
        }
        let mut degrees = self.extensions.iter().map(|v| v.degree).collect::<Vec<_>>();
        degrees.sort_unstable();
        degrees.dedup();
        if degrees.len() != self.extensions.len() {
            return Err("Duplicate chord extension".into());
        }
        if self.inversion as usize >= self.intervals().len() {
            return Err("Inversion is outside this chord".into());
        }
        Ok(())
    }

    pub fn root_offset(&self) -> u8 {
        (MAJOR[self.degree as usize] as i16 + self.alteration as i16).rem_euclid(12) as u8
    }
    pub fn root(&self, key: Note) -> Note {
        key.interval(self.degree + 1, self.root_offset() as i16)
    }
    pub fn intervals(&self) -> Vec<Interval> {
        let mut tones = vec![
            Interval {
                degree: 1,
                semitones: 0,
            },
            Interval {
                degree: match self.third {
                    2 => 2,
                    5 => 4,
                    _ => 3,
                },
                semitones: self.third,
            },
            Interval {
                degree: 5,
                semitones: self.fifth,
            },
        ];
        if let Some(semitones) = self.seventh {
            tones.push(Interval {
                degree: 7,
                semitones,
            });
        }
        tones.extend(&self.extensions);
        tones
    }
    pub fn notes(&self, key: Note) -> Vec<Note> {
        let root = self.root(key);
        self.intervals()
            .iter()
            .map(|v| root.interval(v.degree, v.semitones as i16))
            .collect()
    }
    pub fn pitch_classes(&self) -> Vec<u8> {
        self.intervals()
            .iter()
            .map(|v| (self.root_offset() + v.semitones) % 12)
            .collect()
    }
    pub fn is_diatonic(&self, mode: Mode) -> bool {
        self.pitch_classes()
            .iter()
            .all(|p| mode.intervals().contains(p))
    }
    pub fn is_minor(&self) -> bool {
        self.third == 3
    }

    fn suffix(&self, roman: bool) -> String {
        let dim = self.third == 3 && self.fifth == 6;
        let mut value = if dim {
            match (roman, self.seventh) {
                (true, Some(10)) => "ø".into(),
                (true, _) => "°".into(),
                (false, Some(10)) => "m".into(),
                _ => "dim".into(),
            }
        } else if self.fifth == 8 {
            if roman {
                "+".into()
            } else {
                "aug".into()
            }
        } else if self.is_minor() && !roman {
            "m".into()
        } else {
            String::new()
        };
        let natural_extension = self
            .extensions
            .iter()
            .filter(|v| matches!((v.degree, v.semitones), (9, 14) | (11, 17) | (13, 21)))
            .map(|v| v.degree)
            .max();
        if let Some(seventh) = self.seventh {
            if seventh == 11 {
                value.push_str("maj");
            }
            value.push_str(&natural_extension.unwrap_or(7).to_string());
            if dim && seventh == 10 && !roman {
                value.push_str("b5");
            }
        }
        if self.third == 2 {
            value.push_str("sus2");
        }
        if self.third == 5 {
            value.push_str("sus4");
        }
        for interval in &self.extensions {
            if self.seventh.is_some()
                && matches!(
                    (interval.degree, interval.semitones),
                    (9, 14) | (11, 17) | (13, 21)
                )
            {
                continue;
            }
            value.push_str(match (interval.degree, interval.semitones) {
                (6, 9) => "add6",
                (9, 14) => "add9",
                (9, 13) => "(b9)",
                (9, 15) => "(#9)",
                (11, 17) => "add11",
                (11, 18) => "(#11)",
                (13, 20) => "(b13)",
                (13, 21) => "add13",
                _ => "",
            });
        }
        value
    }

    pub fn name(&self, key: Note) -> String {
        let mut name = format!("{}{}", self.root(key).name(), self.suffix(false));
        if self.inversion > 0 {
            name.push('/');
            name.push_str(&self.notes(key)[self.inversion as usize].name());
        }
        name
    }
    pub fn roman(&self) -> String {
        let numeral = ["I", "II", "III", "IV", "V", "VI", "VII"][self.degree as usize];
        let numeral = if self.is_minor() {
            numeral.to_lowercase()
        } else {
            numeral.to_string()
        };
        let accidental = if self.alteration < 0 {
            "b".repeat((-self.alteration) as usize)
        } else {
            "#".repeat(self.alteration as usize)
        };
        let mut text = format!("{accidental}{numeral}{}", self.suffix(true));
        if self.inversion > 0 {
            text.push_str(&format!("(inv{})", self.inversion));
        }
        text
    }
    pub fn function(&self) -> &'static str {
        if self.source == ChordSource::SecondaryDominant {
            return "Applied dominant";
        }
        if self.source == ChordSource::Borrowed {
            return "Modal colour";
        }
        match self.degree {
            0 => "Tonic",
            1 | 3 => "Predominant",
            4 | 6 => "Dominant colour",
            _ => "Tonic colour",
        }
    }
}

/// Parses the explicitly supported beta dialect, rejecting unknown trailing text.
pub fn parse_roman(text: &str) -> Result<Chord, String> {
    let mut tail = text.trim();
    let mut alteration = 0;
    while let Some(ch) = tail.chars().next() {
        match ch {
            'b' => alteration -= 1,
            '#' => alteration += 1,
            _ => break,
        }
        tail = &tail[1..];
    }
    let size = tail
        .chars()
        .take_while(|c| matches!(c, 'I' | 'V' | 'i' | 'v'))
        .count();
    let numeral = &tail[..size];
    let degree = ["I", "II", "III", "IV", "V", "VI", "VII"]
        .iter()
        .position(|v| *v == numeral.to_ascii_uppercase())
        .ok_or("Invalid Roman numeral")? as u8;
    if numeral != numeral.to_ascii_lowercase() && numeral != numeral.to_ascii_uppercase() {
        return Err("Mixed-case Roman numeral".into());
    }
    tail = &tail[size..];
    let mut chord = Chord {
        degree,
        alteration,
        third: if numeral == numeral.to_ascii_lowercase() {
            3
        } else {
            4
        },
        fifth: 7,
        seventh: None,
        extensions: vec![],
        inversion: 0,
        source: ChordSource::Diatonic,
    };
    if tail.starts_with('ø') {
        chord.fifth = 6;
        chord.third = 3;
        chord.seventh = Some(10);
        tail = &tail['ø'.len_utf8()..];
    } else if tail.starts_with('°') {
        chord.fifth = 6;
        chord.third = 3;
        tail = &tail['°'.len_utf8()..];
    } else if tail.starts_with('+') {
        chord.fifth = 8;
        tail = &tail[1..];
    }
    let major_seventh = tail.starts_with("maj");
    if major_seventh {
        tail = &tail[3..];
    }
    if tail.starts_with("6/4") {
        chord.inversion = 2;
        tail = &tail[3..];
    } else if tail.starts_with('6') {
        chord.inversion = 1;
        tail = &tail[1..];
    } else {
        for n in [13, 11, 9, 7] {
            let token = n.to_string();
            if tail.starts_with(&token) {
                chord.seventh = Some(if major_seventh {
                    11
                } else if chord.fifth == 6 && chord.seventh.is_none() {
                    9
                } else {
                    10
                });
                if n >= 9 {
                    chord.extensions.push(Interval {
                        degree: 9,
                        semitones: 14,
                    });
                }
                if n >= 11 {
                    chord.extensions.push(Interval {
                        degree: 11,
                        semitones: 17,
                    });
                }
                if n >= 13 {
                    chord.extensions.push(Interval {
                        degree: 13,
                        semitones: 21,
                    });
                }
                tail = &tail[token.len()..];
                break;
            }
        }
    }
    if major_seventh && chord.seventh != Some(11) {
        return Err("maj must be followed by 7, 9, 11, or 13".into());
    }
    for (token, third) in [("sus2", 2), ("sus4", 5)] {
        if tail.starts_with(token) {
            chord.third = third;
            tail = &tail[token.len()..];
        }
    }
    while !tail.is_empty() {
        let mut matched = false;
        for (token, degree, semitones) in [
            ("add6", 6, 9),
            ("add9", 9, 14),
            ("add11", 11, 17),
            ("add13", 13, 21),
            ("(b9)", 9, 13),
            ("(#9)", 9, 15),
            ("(#11)", 11, 18),
            ("(b13)", 13, 20),
        ] {
            if tail.starts_with(token) {
                chord.extensions.push(Interval { degree, semitones });
                tail = &tail[token.len()..];
                matched = true;
                break;
            }
        }
        if matched {
            continue;
        }
        if tail.starts_with("(inv") && tail.ends_with(')') {
            chord.inversion = tail[4..tail.len() - 1]
                .parse()
                .map_err(|_| "Invalid inversion")?;
            tail = "";
        } else {
            return Err(format!("Unsupported Roman suffix: {tail}"));
        }
    }
    chord.validate()?;
    Ok(chord)
}

pub fn common_tones(a: &Chord, b: &Chord) -> usize {
    let pitches = a.pitch_classes();
    b.pitch_classes()
        .iter()
        .filter(|p| pitches.contains(p))
        .count()
}

pub fn pitch_distance(a: u8, b: u8) -> u8 {
    let d = a.abs_diff(b) % 12;
    d.min(12 - d)
}

/// Keep the selected bass underneath a complete upper voicing, then search octaves.
pub fn voice(chord: &Chord, key: Note, previous: &[u8], open: bool) -> Vec<u8> {
    let tones = chord.notes(key);
    let bass = tones[chord.inversion as usize].pitch_class();
    let previous_bass = previous.first().copied().unwrap_or(45);
    let bass_midi = [bass + 36, bass + 48]
        .into_iter()
        .min_by_key(|n| n.abs_diff(previous_bass))
        .unwrap();
    let mut best = vec![];
    let mut best_cost = f64::INFINITY;
    for rotation in 0..tones.len() {
        for floor in [52u8, 57, 60] {
            let mut upper = vec![];
            let mut minimum = floor.max(bass_midi + 4);
            for i in 0..tones.len() {
                let pc = tones[(i + rotation) % tones.len()].pitch_class();
                let mut midi = pc + 12 * (minimum / 12);
                if midi < minimum {
                    midi += 12;
                }
                upper.push(midi);
                minimum = midi + 1;
            }
            if open && upper.len() >= 3 && upper[0] >= bass_midi + 16 {
                upper[0] -= 12;
            }
            upper.sort_unstable();
            let mut candidate = vec![bass_midi];
            candidate.extend(upper);
            let cost: f64 = candidate
                .iter()
                .enumerate()
                .map(|(i, n)| {
                    let target = previous.get(i).copied().unwrap_or(48 + i as u8 * 6);
                    n.abs_diff(target) as f64 + if *n > 88 { (*n - 88) as f64 * 3.0 } else { 0.0 }
                })
                .sum();
            if cost < best_cost {
                best_cost = cost;
                best = candidate;
            }
        }
    }
    best
}

#[cfg(test)]
mod unit {
    use super::*;
    #[test]
    fn c_major_triads() {
        let key = Note::parse("C").unwrap();
        let expected = ["C", "Dm", "Em", "F", "G", "Am", "Bdim"];
        for (i, name) in expected.iter().enumerate() {
            assert_eq!(
                Chord::diatonic(Mode::Major, i as u8, false).name(key),
                *name
            );
        }
    }
    #[test]
    fn spelled_intervals_and_minor() {
        let sharp = Note::parse("F#").unwrap();
        assert_eq!(
            Chord::diatonic(Mode::Major, 6, false)
                .notes(sharp)
                .iter()
                .map(|n| n.name())
                .collect::<Vec<_>>(),
            ["E#", "G#", "B"]
        );
        let flat = Note::parse("Gb").unwrap();
        assert_eq!(
            Chord::diatonic(Mode::Major, 3, false).root(flat).name(),
            "Cb"
        );
        assert_eq!(Chord::diatonic(Mode::Minor, 2, false).roman(), "bIII");
    }
    #[test]
    fn roman_round_trip_and_extensions() {
        for mode in [
            Mode::Major,
            Mode::Minor,
            Mode::Dorian,
            Mode::Phrygian,
            Mode::Lydian,
            Mode::Mixolydian,
            Mode::Locrian,
        ] {
            for degree in 0..7 {
                for seventh in [false, true] {
                    let chord = Chord::diatonic(mode, degree, seventh);
                    assert_eq!(parse_roman(&chord.roman()).unwrap(), chord);
                }
            }
        }
        let chord = parse_roman("Imaj9").unwrap();
        assert_eq!(chord.name(Note::parse("D").unwrap()), "Dmaj9");
        assert_eq!(
            parse_roman("I6/4").unwrap().name(Note::parse("D").unwrap()),
            "D/A"
        );
        assert_eq!(
            parse_roman("ivadd6")
                .unwrap()
                .notes(Note::parse("D").unwrap())
                .iter()
                .map(|n| n.name())
                .collect::<Vec<_>>(),
            ["G", "Bb", "D", "E"]
        );
        assert_eq!(
            parse_roman("V7(b9)")
                .unwrap()
                .name(Note::parse("C").unwrap()),
            "G7(b9)"
        );
        for bad in [
            "",
            "VIII",
            "imaj",
            "Igarbage",
            "I(inv9)",
            "Iadd9add9",
            "###I",
            "IIII",
            "Iv",
        ] {
            assert!(parse_roman(bad).is_err(), "{bad}");
        }
    }
    #[test]
    fn octave_transposition_and_voicing() {
        for pc in 0..12 {
            for mode in [Mode::Major, Mode::Minor, Mode::Dorian] {
                for degree in 0..7 {
                    let key = Note::from_pitch(pc, false);
                    let chord = Chord::diatonic(mode, degree, true);
                    let notes = chord.notes(key);
                    let voiced = voice(&chord, key, &[], true);
                    assert!(voiced.windows(2).all(|w| w[0] < w[1]));
                    assert!(voiced.iter().all(|n| *n < 128));
                    for note in notes {
                        assert!(voiced.iter().any(|n| n % 12 == note.pitch_class()));
                    }
                    assert_eq!(Note::from_pitch(pc + 12, false), key);
                }
            }
        }
    }
}
