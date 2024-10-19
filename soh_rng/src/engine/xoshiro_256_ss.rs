use super::Engine64;

#[derive(Default)]
pub struct Xoshiro256SS {
    state: [u64; 4],
}

impl Engine64 for Xoshiro256SS {
    fn set_seed(&mut self, seed: u64) {
        let mut sm = super::SplitMix::new(seed);
        self.state.iter_mut().for_each(|s| *s = sm.gen::<u64>());
    }

    fn next(&mut self) -> u64 {
        let res = self.state[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);

        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;

        self.state[3] = self.state[3].rotate_left(45);

        return res;
    }
}
