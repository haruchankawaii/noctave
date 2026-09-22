use crate::style::Profile;
use crate::{Cadence, Energy, Mood, Request, Score};
use harmony_core::{common_tones, pitch_distance, Chord, ChordSource};
pub fn target(request: &Request, index: usize) -> f64 {
    let x = index as f64 / (request.length - 1) as f64;
    match request.energy {
        Energy::Steady => 0.35,
        Energy::SlowBuild => {
            if x < 0.82 {
                0.15 + 0.65 * x / 0.82
            } else {
                0.8 - (x - 0.82) / 0.18 * 0.4
            }
        }
        Energy::Wave => 0.2 + 0.45 * (x * std::f64::consts::PI).sin(),
        Energy::Resolve => 0.7 * (1.0 - x) + 0.1,
    }
}
pub fn tension(
    chord: &Chord,
    previous: Option<&Chord>,
    request: &Request,
    profile: &Profile,
) -> f64 {
    let base = match chord.degree {
        0 => 0.08,
        1 => 0.4,
        2 => 0.3,
        3 => 0.28,
        4 => 0.62,
        5 => 0.2,
        _ => 0.68,
    };
    let chromatic = chord
        .pitch_classes()
        .iter()
        .filter(|p| !request.mode.intervals().contains(p))
        .count() as f64
        * 0.14;
    let distance = previous
        .map(|p| pitch_distance(chord.root_offset(), p.root_offset()) as f64 * 0.018)
        .unwrap_or(0.0);
    let extension =
        if chord.seventh.is_some() { 0.08 } else { 0.0 } + chord.extensions.len() as f64 * 0.045;
    let familiarity = (chord.extensions.len() as f64 * profile.weight("extensions")
        + if chord.third == 2 || chord.third == 5 {
            profile.weight("suspended")
        } else {
            0.0
        })
        * 0.045;
    (base + chromatic + distance + extension + chord.inversion as f64 * 0.018 - familiarity)
        .clamp(0.0, 1.0)
}
fn movement(previous: &Chord, chord: &Chord, profile: &Profile) -> f64 {
    let distance = pitch_distance(previous.root_offset(), chord.root_offset());
    let common = common_tones(previous, chord) as f64
        / previous.intervals().len().max(chord.intervals().len()) as f64;
    let mut value = common * profile.weight("common_tone") * 1.5;
    if distance == 1 || distance == 2 {
        value += profile.weight("stepwise") * 0.75;
    }
    if distance == 5 {
        value += profile.weight("fifths") * 0.85;
    }
    if previous.source == ChordSource::SecondaryDominant
        && (chord.root_offset() + 7) % 12 == previous.root_offset()
    {
        value += 1.25;
    }
    let a = previous.pitch_classes();
    let b = chord.pitch_classes();
    let cost = a
        .iter()
        .map(|p| b.iter().map(|q| pitch_distance(*p, *q)).min().unwrap_or(6) as f64)
        .sum::<f64>()
        / a.len() as f64;
    value + (1.0 - cost / 6.0) * 0.5
}
pub fn evaluate(
    chord: &Chord,
    history: &[Chord],
    request: &Request,
    profile: &Profile,
    index: usize,
) -> Score {
    let previous = history.last();
    let theory = if chord.source == ChordSource::Diatonic {
        0.7
    } else {
        0.1 + profile.weight("borrowed") * 0.6
    };
    let mut style = if chord.seventh.is_some() || !chord.extensions.is_empty() {
        profile.weight("extensions") * 1.1
    } else {
        (1.0 - profile.weight("extensions")) * 0.8
    };
    if chord.third == 2 || chord.third == 5 {
        style += profile.weight("suspended") * 0.8;
    }
    if chord.source != ChordSource::Diatonic {
        style += (profile.weight("borrowed") - 0.5) * 1.2;
    }
    style += match request.mood {
        Mood::Melancholic => {
            if chord.is_minor() {
                0.55
            } else {
                0.0
            }
        }
        Mood::Hopeful => {
            if chord.third == 4 && chord.fifth == 7 {
                0.45
            } else {
                0.0
            }
        }
        Mood::Dreamy => {
            if !chord.extensions.is_empty() || chord.third == 2 || chord.third == 5 {
                0.4
            } else {
                0.0
            }
        }
        Mood::Restless => {
            if chord.source != ChordSource::Diatonic || chord.degree == 4 || chord.degree == 6 {
                0.45
            } else {
                0.0
            }
        }
        Mood::Neutral => 0.0,
    };
    let mut move_score = previous.map(|p| movement(p, chord, profile)).unwrap_or(0.0);
    if let Some(Some(next)) = request.locks.get(index + 1) {
        move_score += movement(chord, next, profile) * 0.8;
    }
    let tension_score =
        (1.0 - (tension(chord, previous, request, profile) - target(request, index)).abs()) * 1.6;
    let occurrences = history
        .iter()
        .filter(|c| c.root_offset() == chord.root_offset())
        .count();
    let mut repetition = if occurrences == 0 {
        0.4
    } else {
        -(occurrences as f64) * (1.0 - profile.weight("repetition")) * 0.65
    };
    if previous.is_some_and(|p| p.root_offset() == chord.root_offset()) {
        repetition -= 1.5;
    }
    if history.len() >= 2 && history[history.len() - 2].root_offset() == chord.root_offset() {
        repetition -= 0.25;
    }
    if index >= 4 && history.get(index - 4).is_some_and(|p| p == chord) {
        repetition += profile.weight("repetition") * 0.5;
    }
    let mut phrase = 0.0;
    if index == 0 {
        phrase += if chord.degree == 0 {
            1.2
        } else if chord.degree == 5 {
            0.35
        } else {
            0.0
        };
    }
    if index + 1 == request.length as usize {
        phrase += match request.cadence {
            Cadence::Resolved => {
                if chord.degree == 0 && chord.alteration == 0 {
                    2.3
                } else {
                    -0.8
                }
            }
            Cadence::Open => {
                if chord.degree != 0 {
                    0.4 * (1.0 - profile.weight("resolution"))
                } else {
                    -0.15
                }
            }
            Cadence::Loop => history
                .first()
                .map(|first| movement(chord, first, profile))
                .unwrap_or(0.0),
        };
    }
    if index + 2 == request.length as usize
        && request.cadence == Cadence::Resolved
        && [3, 4, 6].contains(&chord.degree)
    {
        phrase += 0.8;
    }
    let total = theory + style + move_score + tension_score + repetition + phrase;
    Score {
        theory,
        style,
        movement: move_score,
        tension: tension_score,
        repetition,
        phrase,
        total,
    }
}
pub fn explain(
    chord: &Chord,
    history: &[Chord],
    request: &Request,
    profile: &Profile,
    index: usize,
    score: &Score,
    locked: bool,
) -> Vec<String> {
    let mut reasons = vec![match chord.source {
        ChordSource::Diatonic => format!(
            "Built from the {} scale's chord vocabulary.",
            request.mode.label()
        ),
        ChordSource::Borrowed => {
            "Borrowed from the parallel mode; adds a contrasting harmonic colour.".into()
        }
        ChordSource::SecondaryDominant => {
            "An applied dominant candidate; a downward-fifth resolution receives a movement bonus."
                .into()
        }
    }];
    if locked {
        reasons.push("This relative chord and its inversion were preserved by your lock.".into());
    }
    if let Some(previous) = history.last() {
        let common = common_tones(previous, chord);
        reasons.push(format!("Shares {common} pitch class{} with the previous chord; common tones contribute to movement scoring.",if common==1{""}else{"es"}));
        let distance = pitch_distance(previous.root_offset(), chord.root_offset());
        if distance <= 2 && distance > 0 {
            reasons.push(
                "The root moves by a step, receiving this profile's stepwise-motion bonus.".into(),
            );
        }
    }
    if chord.seventh.is_some() || !chord.extensions.is_empty() {
        reasons.push(format!(
            "Extended colour is weighted {:.0}% in the resolved style profile.",
            profile.weight("extensions") * 100.0
        ));
    }
    if request.locks.get(index + 1).is_some_and(Option::is_some) {
        reasons.push(
            "The transition into the following locked chord was included in the search score."
                .into(),
        );
    }
    reasons.push(format!(
        "Contextual tension {:.0}% against a {:.0}% target; tension contribution {:.2}.",
        tension(chord, history.last(), request, profile) * 100.0,
        target(request, index) * 100.0,
        score.tension
    ));
    if chord.inversion > 0 {
        reasons.push(format!("Inversion {} places a chord tone in the bass; unlocked voicings are chosen for economical voice movement.",chord.inversion));
    }
    reasons
}
