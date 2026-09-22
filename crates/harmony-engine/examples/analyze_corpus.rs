//! Read-only corpus validator and aggregate reporter. No network or data import.
use harmony_core::{parse_roman, Mode, Note};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs, process,
};
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Corpus {
    schema_version: u8,
    style: String,
    synthetic: bool,
    provenance: Provenance,
    tracks: Vec<Track>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Provenance {
    source: String,
    license: String,
    usage: String,
    license_verified: bool,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Track {
    track_id: String,
    artist_id: String,
    sections: Vec<Section>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Section {
    name: String,
    key: String,
    mode: Mode,
    progression: Vec<String>,
}
fn count(map: &mut BTreeMap<String, u64>, key: String) {
    *map.entry(key).or_default() += 1;
}
fn analyze(corpus: Corpus) -> Result<Value, String> {
    if corpus.schema_version != 1 {
        return Err("Unsupported corpus schema".into());
    }
    if !corpus.provenance.license_verified
        || corpus.provenance.license.trim().is_empty()
        || corpus.provenance.source.trim().is_empty()
        || ![
            "RESEARCH_ALLOWED",
            "DATA_IMPORT_ALLOWED",
            "COMMERCIAL_EMBED_ALLOWED",
        ]
        .contains(&corpus.provenance.usage.as_str())
    {
        return Err("A verified license, source, and permitted research usage are required".into());
    }
    if corpus.style.trim().is_empty() || corpus.tracks.is_empty() {
        return Err("A style and at least one track are required".into());
    }
    let mut artists = BTreeSet::new();
    let mut tracks = BTreeSet::new();
    let mut vocabulary = BTreeMap::new();
    let mut transitions = BTreeMap::new();
    let mut endings = BTreeMap::new();
    let mut lengths = BTreeMap::new();
    let mut normalized = vec![];
    let mut chord_count = 0u64;
    for track in corpus.tracks {
        if track.track_id.trim().is_empty()
            || track.artist_id.trim().is_empty()
            || !tracks.insert(track.track_id.clone())
            || track.sections.is_empty()
        {
            return Err(
                "Track identifiers must be nonempty and unique; tracks need sections".into(),
            );
        }
        artists.insert(track.artist_id);
        for section in track.sections {
            let key = Note::parse(&section.key)?;
            if section.progression.is_empty() || section.name.trim().is_empty() {
                return Err("Sections need a name and at least one chord".into());
            }
            let mut romans = vec![];
            let mut absolute = vec![];
            let mut canonical = vec![];
            for text in &section.progression {
                let chord = parse_roman(text)
                    .map_err(|e| format!("{} / {} / {text}: {e}", track.track_id, section.name))?;
                romans.push(chord.roman());
                absolute.push(chord.name(key));
                canonical.push(chord.name(Note::parse("C")?));
                count(&mut vocabulary, chord.roman());
                chord_count += 1;
            }
            for pair in romans.windows(2) {
                count(&mut transitions, format!("{} -> {}", pair[0], pair[1]));
            }
            count(&mut endings, romans.last().unwrap().clone());
            count(&mut lengths, romans.len().to_string());
            normalized.push(json!({"track_id":track.track_id,"section":section.name,"mode":section.mode,"key":section.key,"roman":romans,"absolute":absolute,"in_c":canonical}));
        }
    }
    let transition_total = transitions.values().sum::<u64>();
    Ok(
        json!({"schema_version":1,"style":corpus.style,"synthetic":corpus.synthetic,"source":corpus.provenance.source,"license":corpus.provenance.license,
        "research_status":"experimental","validation":"No automatic confidence or production-ready claim; review the corpus and listening evidence.",
        "sample_tracks":tracks.len(),"sample_artists":artists.len(),"sections":normalized.len(),"chords":chord_count,
        "chord_counts":vocabulary,"transition_counts":transitions,"transition_total":transition_total,"section_ending_counts":endings,"progression_lengths":lengths,"normalized_sections":normalized}),
    )
}
fn run() -> Result<(), String> {
    let path = env::args()
        .nth(1)
        .ok_or("Usage: cargo run -p harmony-engine --example analyze_corpus -- corpus.json")?;
    if fs::metadata(&path).map_err(|e| e.to_string())?.len() > 10_485_760 {
        return Err("Corpus exceeds the beta 10 MB limit".into());
    }
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    let result = analyze(serde_json::from_slice(&bytes).map_err(|e| e.to_string())?)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&result).map_err(|e| e.to_string())?
    );
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("Corpus: {error}");
        process::exit(1);
    }
}
#[cfg(test)]
mod unit {
    use super::*;
    const EXAMPLE: &str = include_str!("../../../research/example-corpus.json");
    #[test]
    fn synthetic_is_explicit_and_transitions_stop_at_section_boundaries() {
        let result = analyze(serde_json::from_str(EXAMPLE).unwrap()).unwrap();
        assert_eq!(result["synthetic"], true);
        assert_eq!(result["sample_tracks"], 2);
        assert_eq!(result["sample_artists"], 2);
        assert_eq!(result["chords"], 12);
        assert_eq!(result["transition_total"], 9);
    }
    #[test]
    fn unverified_sources_and_duplicate_tracks_are_rejected() {
        let mut corpus: Corpus = serde_json::from_str(EXAMPLE).unwrap();
        corpus.provenance.license_verified = false;
        assert!(analyze(corpus).is_err());
        let mut corpus: Corpus = serde_json::from_str(EXAMPLE).unwrap();
        corpus.tracks[1].track_id = corpus.tracks[0].track_id.clone();
        assert!(analyze(corpus).is_err());
    }
}
