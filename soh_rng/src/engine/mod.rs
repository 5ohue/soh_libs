mod traits;

mod lcg;
mod xoshiro_128_ss;

mod split_mix;
mod xoshiro_256_ss;

pub mod generators {
    // 32 bit generators
    pub type LCG = super::lcg::LCG<1664525, 1013904223>;
    pub use super::xoshiro_128_ss::Xoshiro128SS;

    // 64 bit generators
    pub use super::split_mix::SplitMix;
    pub use super::xoshiro_256_ss::Xoshiro256SS;
}
use generators::*;

// Traits
pub use traits::*;
