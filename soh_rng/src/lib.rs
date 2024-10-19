mod engine;
mod gen_trait;

pub mod prelude;

pub use engine::{Engine32, Engine64};
pub use engine::generators::*;

pub type RNG32 = Lcg;
pub type RNG64 = SplitMix;

#[cfg(test)]
mod tests {
    use super::*;

    const NUM_OF_TRIES: usize = 10_000;

    #[test]
    fn test_float() {
        let mut rng_32 = RNG32::new(0xdeadbeef);

        for _ in 0..NUM_OF_TRIES {
            let rand_f32: f32 = rng_32.gen();
            let rand_f64: f64 = rng_32.gen();

            assert!(rand_f32 >= 0.0);
            assert!(rand_f32 <= 1.0);
            assert!(rand_f64 >= 0.0);
            assert!(rand_f64 <= 1.0);
        }

        let mut rng_64 = RNG64::new(0xdeadbeef);

        for _ in 0..NUM_OF_TRIES {
            let rand_f32: f32 = rng_64.gen();
            let rand_f64: f64 = rng_64.gen();

            assert!(rand_f32 >= 0.0);
            assert!(rand_f32 <= 1.0);
            assert!(rand_f64 >= 0.0);
            assert!(rand_f64 <= 1.0);
        }
    }
}
