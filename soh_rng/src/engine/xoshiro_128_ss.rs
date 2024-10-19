use super::Engine32;

#[derive(Default)]
pub struct Xoshiro128SS {
    state: [u32; 4],
}

impl Engine32 for Xoshiro128SS {
    fn set_seed(&mut self, seed: u32) {
        let mut lcg = super::Lcg::new(seed);
        self.state.iter_mut().for_each(|s| *s = lcg.gen::<u32>());
    }

    fn next(&mut self) -> u32 {
        let res = self.state[1].wrapping_mul(5).rotate_left(7).wrapping_mul(9);

        let t = self.state[1] << 9;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] ^= t;

        self.state[3] = self.state[3].rotate_left(11);

        return res;
    }
}
