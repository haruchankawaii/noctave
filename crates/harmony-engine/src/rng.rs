/// SplitMix64 with fixed conversion: reproducible for this engine version.
pub struct Rng(u64);
impl Rng {
    pub fn new(seed: u32) -> Self {
        Self(seed as u64)
    }
    pub fn unit(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^= z >> 31;
        (z >> 11) as f64 / ((1u64 << 53) as f64)
    }
    pub fn weighted(&mut self, scores: &[f64], temperature: f64) -> usize {
        let maximum = scores.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let weights: Vec<_> = scores
            .iter()
            .map(|s| ((s - maximum) / temperature).exp())
            .collect();
        let mut draw = self.unit() * weights.iter().sum::<f64>();
        for (i, weight) in weights.iter().enumerate() {
            draw -= weight;
            if draw <= 0.0 {
                return i;
            }
        }
        weights.len() - 1
    }
}
