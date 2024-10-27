mod engine;
mod gen_trait;

pub mod prelude;

pub use engine::generators::*;
pub use engine::{Engine32, Engine64};

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

    #[test]
    fn test_permutation_table_32() {
        fn test_func<TRng: crate::Engine32>() {
            let mut rng = TRng::new(0xdeadbeef);

            let table_size = rng.gen_range(20, 1000);

            let mut arr = (0..table_size).collect::<Vec<_>>();
            rng.shuffle(&mut arr);

            let mut set = std::collections::HashSet::new();
            for &i in arr.iter() {
                set.insert(i);
            }

            assert!(set.len() == arr.len());
        }

        for _ in 0..100 {
            test_func::<Lcg>();
            test_func::<Xoshiro128SS>();
        }
    }

    #[test]
    fn test_permutation_table_64() {
        fn test_func<TRng: crate::Engine64>() {
            let mut rng = TRng::new(0xdeadbeef);

            let table_size = rng.gen_range(20, 1000);

            let mut arr = (0..table_size).collect::<Vec<_>>();
            rng.shuffle(&mut arr);

            let mut set = std::collections::HashSet::new();
            for &i in arr.iter() {
                set.insert(i);
            }

            assert!(set.len() == arr.len());
        }

        for _ in 0..100 {
            test_func::<SplitMix>();
            test_func::<Xoshiro256SS>();
        }
    }
}
