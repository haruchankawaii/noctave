use crate::Progression;
fn variable(mut value: u32) -> Vec<u8> {
    let mut bytes = vec![(value & 127) as u8];
    value >>= 7;
    while value > 0 {
        bytes.push(((value & 127) | 128) as u8);
        value >>= 7;
    }
    bytes.reverse();
    bytes
}
/// Standard MIDI type 0, 480 PPQN, 4/4, acoustic piano.
pub fn export(progression: &Progression) -> Result<Vec<u8>, String> {
    progression.request.validate()?;
    if progression.chords.len() != progression.request.length as usize {
        return Err("Chord count does not match request".into());
    }
    let tempo = 60_000_000u32 / progression.request.tempo as u32;
    let mut track = vec![
        0,
        0xff,
        0x51,
        3,
        ((tempo >> 16) & 255) as u8,
        ((tempo >> 8) & 255) as u8,
        (tempo & 255) as u8,
        0,
        0xff,
        0x58,
        4,
        4,
        2,
        24,
        8,
        0,
        0xc0,
        0,
    ];
    let ticks = progression.request.beats_per_chord as u32 * 480;
    for chord in &progression.chords {
        if chord.midi.is_empty() || chord.midi.len() > 12 || chord.midi.iter().any(|p| *p > 127) {
            return Err("Invalid MIDI voicing".into());
        }
        for pitch in &chord.midi {
            track.extend([0, 0x90, *pitch, 76]);
        }
        for (i, pitch) in chord.midi.iter().enumerate() {
            track.extend(variable(if i == 0 { ticks } else { 0 }));
            track.extend([0x80, *pitch, 0]);
        }
    }
    track.extend([0, 0xff, 0x2f, 0]);
    let mut file = b"MThd".to_vec();
    file.extend(6u32.to_be_bytes());
    file.extend([0, 0, 0, 1, 1, 224]);
    file.extend(b"MTrk");
    file.extend((track.len() as u32).to_be_bytes());
    file.extend(track);
    Ok(file)
}
#[cfg(test)]
mod unit {
    use super::*;
    #[test]
    fn valid_midi_header_and_track_size() {
        let p = crate::generate(crate::Request {
            length: 2,
            ..Default::default()
        })
        .unwrap();
        let bytes = export(&p).unwrap();
        assert_eq!(&bytes[..4], b"MThd");
        assert_eq!(&bytes[14..18], b"MTrk");
        assert_eq!(
            u32::from_be_bytes(bytes[18..22].try_into().unwrap()) as usize,
            bytes.len() - 22
        );
        assert_eq!(&bytes[bytes.len() - 4..], &[0, 255, 47, 0]);
    }
}
