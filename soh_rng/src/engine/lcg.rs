use super::Engine32;

#[derive(Default)]
pub struct LCG<const MUL: u32, const ADD: u32> {
    state: u32,
}

impl<const MUL: u32, const ADD: u32> Engine32 for LCG<MUL, ADD> {
    fn set_seed(&mut self, seed: u32) {
        self.state = seed;
    }

    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(MUL).wrapping_add(ADD);
        return self.state;
    }
}
