use crate::gen_trait::*;

pub trait Engine32: Default {
    fn set_seed(&mut self, seed: u32);
    fn next(&mut self) -> u32;

    fn new(seed: u32) -> Self {
        let mut rng = Self::default();
        rng.set_seed(seed);
        return rng;
    }

    fn new_from_time() -> Self {
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        return Self::new(t as u32);
    }

    fn gen<T: RandomlyGenerated32>(&mut self) -> T {
        return RandomlyGenerated32::from_rand_32(self.next());
    }

    fn gen_to<T: RandomlyGenerated32>(&mut self, to: T) -> T {
        return RandomlyGenerated32::from_rand_32_to(self.next(), to);
    }

    fn gen_range<T: RandomlyGenerated32>(&mut self, from: T, to: T) -> T {
        return RandomlyGenerated32::from_rand_32_range(self.next(), from, to);
    }

    /// Fisher-Yates shuffle
    fn shuffle<T>(&mut self, array: &mut [T]) {
        for i in (0..array.len()).rev() {
            let j = self.gen_to(i as u32 + 1);
            array.swap(i, j as usize);
        }
    }
}

pub trait Engine64: Default {
    fn set_seed(&mut self, seed: u64);
    fn next(&mut self) -> u64;

    fn new(seed: u64) -> Self {
        let mut rng = Self::default();
        rng.set_seed(seed);
        return rng;
    }

    fn new_from_time() -> Self {
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        return Self::new(t);
    }

    fn gen<T: RandomlyGenerated64>(&mut self) -> T {
        return RandomlyGenerated64::from_rand_64(self.next());
    }

    fn gen_to<T: RandomlyGenerated64>(&mut self, to: T) -> T {
        return RandomlyGenerated64::from_rand_64_to(self.next(), to);
    }

    fn gen_range<T: RandomlyGenerated64>(&mut self, from: T, to: T) -> T {
        return RandomlyGenerated64::from_rand_64_range(self.next(), from, to);
    }

    /// Fisher-Yates shuffle
    fn shuffle<T>(&mut self, array: &mut [T]) {
        for i in (0..array.len()).rev() {
            let j = self.gen_to(i + 1);
            array.swap(i, j);
        }
    }
}
