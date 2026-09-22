use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StyleDefinition {
    pub id: String,
    pub name: String,
    pub parent: Option<String>,
    pub description: String,
    pub weights: BTreeMap<String, f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub lineage: Vec<String>,
    pub weights: BTreeMap<String, f64>,
    pub research_status: String,
    pub confidence: f64,
    pub sample_tracks: u32,
    pub sample_artists: u32,
}
impl Profile {
    pub fn weight(&self, name: &str) -> f64 {
        self.weights.get(name).copied().unwrap_or(0.5)
    }
}
pub fn definitions() -> Vec<StyleDefinition> {
    serde_json::from_str(include_str!("../../../data/styles/profiles.json"))
        .expect("validated embedded style data")
}
pub fn resolve(id: &str) -> Result<Profile, String> {
    let data = definitions();
    let leaf = data
        .iter()
        .find(|p| p.id == id)
        .ok_or("Unknown style profile")?;
    let mut ancestors = vec![];
    let mut visited = BTreeSet::new();
    let mut current = Some(leaf);
    while let Some(item) = current {
        if !visited.insert(item.id.clone()) {
            return Err("Style inheritance cycle".into());
        }
        ancestors.push(item);
        current = match &item.parent {
            Some(parent) => Some(
                data.iter()
                    .find(|p| p.id == *parent)
                    .ok_or("Missing style parent")?,
            ),
            None => None,
        };
    }
    ancestors.reverse();
    let mut weights = BTreeMap::new();
    for item in &ancestors {
        for (name, value) in &item.weights {
            if !value.is_finite() || !(0.0..=1.0).contains(value) {
                return Err("Profile weights must be between zero and one".into());
            }
            weights.insert(name.clone(), *value);
        }
    }
    Ok(Profile {
        id: id.into(),
        name: leaf.name.clone(),
        lineage: ancestors.iter().map(|p| p.name.clone()).collect(),
        weights,
        research_status: "experimental".into(),
        confidence: 0.0,
        sample_tracks: 0,
        sample_artists: 0,
    })
}
#[cfg(test)]
mod unit {
    use super::*;
    #[test]
    fn every_profile_resolves_and_inherits() {
        for item in definitions() {
            let profile = resolve(&item.id).unwrap();
            assert_eq!(profile.weights.len(), 9);
            assert_eq!(profile.confidence, 0.0);
        }
        let japan = resolve("post-rock-japan").unwrap();
        assert_eq!(
            japan.weight("suspended"),
            resolve("post-rock").unwrap().weight("suspended")
        );
        assert_eq!(japan.lineage.len(), 3);
        assert!(resolve("unknown").is_err());
    }
}
