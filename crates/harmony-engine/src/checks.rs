use super::*;
#[test]
fn restoring_does_not_invent_locks_or_change_score() {
    let first = generate(Request::default()).unwrap();
    let restored = transpose(&first, &first.request.key).unwrap();
    assert_eq!(
        serde_json::to_value(first).unwrap(),
        serde_json::to_value(restored).unwrap()
    );
}
#[test]
fn resolved_ending_and_simple_complexity_obey_constraints() {
    for mode in [
        Mode::Major,
        Mode::Minor,
        Mode::Dorian,
        Mode::Lydian,
        Mode::Locrian,
    ] {
        let result = generate(Request {
            mode,
            cadence: Cadence::Resolved,
            complexity: Complexity::Simple,
            creativity: 100,
            length: 4,
            ..Default::default()
        })
        .unwrap();
        let last = &result.chords.last().unwrap().chord;
        assert_eq!(last.degree, 0);
        assert_eq!(last.alteration, 0);
        assert_eq!(last.third, mode.intervals()[2]);
        assert!(result
            .chords
            .iter()
            .all(|c| c.chord.seventh.is_none() && c.chord.extensions.is_empty()));
    }
}
#[test]
fn same_seed_same_full_result() {
    let a = generate(Request::default()).unwrap();
    let b = generate(Request::default()).unwrap();
    assert_eq!(
        serde_json::to_string(&a).unwrap(),
        serde_json::to_string(&b).unwrap()
    );
}
#[test]
fn locks_survive_new_seed_including_inversions() {
    let first = generate(Request::default()).unwrap();
    let mut request = first.request.clone();
    request.seed = 87;
    request.locks = first
        .chords
        .iter()
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 0 {
                Some(c.chord.clone())
            } else {
                None
            }
        })
        .collect();
    let next = generate(request).unwrap();
    for i in (0..8).step_by(2) {
        assert_eq!(next.chords[i].chord, first.chords[i].chord);
        assert_eq!(next.chords[i].name, first.chords[i].name);
    }
    assert!(next
        .chords
        .iter()
        .zip(first.chords.iter())
        .any(|(a, b)| a.chord != b.chord));
}
#[test]
fn transpose_preserves_structure_and_locks() {
    let first = generate(Request::default()).unwrap();
    let next = transpose(&first, "E").unwrap();
    for (a, b) in first.chords.iter().zip(next.chords.iter()) {
        assert_eq!(a.chord, b.chord);
        assert_eq!(a.roman, b.roman);
        let p = a.chord.notes(Note::parse("D").unwrap());
        let q = b.chord.notes(Note::parse("E").unwrap());
        for (x, y) in p.iter().zip(q.iter()) {
            assert_eq!((x.pitch_class() + 2) % 12, y.pitch_class());
        }
        assert!(!b.locked);
    }
}
#[test]
fn invalid_requests_are_errors() {
    for request in [
        Request {
            length: 0,
            ..Default::default()
        },
        Request {
            length: 17,
            ..Default::default()
        },
        Request {
            key: "H".into(),
            ..Default::default()
        },
        Request {
            creativity: 101,
            ..Default::default()
        },
        Request {
            tempo: 0,
            ..Default::default()
        },
        Request {
            locks: vec![None],
            ..Default::default()
        },
        Request {
            style: "jazz".into(),
            ..Default::default()
        },
    ] {
        assert!(generate(request).is_err());
    }
}
#[test]
fn conservative_stays_diatonic_and_modes_work() {
    for mode in [
        Mode::Major,
        Mode::Minor,
        Mode::Dorian,
        Mode::Phrygian,
        Mode::Lydian,
        Mode::Mixolydian,
        Mode::Locrian,
    ] {
        let result = generate(Request {
            mode,
            creativity: 0,
            length: 4,
            ..Default::default()
        })
        .unwrap();
        for chord in result.chords {
            assert!(chord.chord.is_diatonic(mode));
            assert!((0.0..=1.0).contains(&chord.tension));
            assert!(chord.midi.windows(2).all(|w| w[0] < w[1]));
        }
    }
}
#[test]
fn styles_seeds_and_traits_change_results() {
    let signature = |r: Request| {
        generate(r)
            .unwrap()
            .chords
            .iter()
            .map(|c| c.roman.clone())
            .collect::<Vec<_>>()
    };
    let default = signature(Request::default());
    assert_ne!(
        default,
        signature(Request {
            style: "rock".into(),
            ..Default::default()
        })
    );
    assert_ne!(
        default,
        signature(Request {
            seed: 101,
            ..Default::default()
        })
    );
    assert_ne!(
        default,
        signature(Request {
            harmonic_trait: Trait::Heavy,
            ..Default::default()
        })
    );
}
#[test]
fn all_locked_is_exact() {
    let first = generate(Request::default()).unwrap();
    let mut request = first.request.clone();
    request.locks = first.chords.iter().map(|c| Some(c.chord.clone())).collect();
    request.seed = 42;
    let next = generate(request).unwrap();
    assert!(first
        .chords
        .iter()
        .zip(next.chords.iter())
        .all(|(a, b)| a.chord == b.chord));
}
