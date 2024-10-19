use super::Engine64;

#[derive(Default)]
pub struct SplitMix {
    state: u64,
}

impl Engine64 for SplitMix {
    fn set_seed(&mut self, seed: u64) {
        self.state = seed;
    }

    fn next(&mut self) -> u64 {
        let mut z: u64 = self.state;

        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);

        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        return z ^ (z >> 31);
    }
}
