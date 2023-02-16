// https://stackoverflow.com/questions/3062746/special-simple-random-number-generator

use serde::{Deserialize, Serialize};

// https://en.wikipedia.org/wiki/Linear_congruential_generator
pub fn linear_congruential_generator(m: usize, a: usize, c: usize, seed: usize) -> usize {
    assert_ne!(seed, 0, "A seed of 0 will always return 0");
    return (a * seed + c) % m;
}

pub fn lcg_sane(seed: usize) -> usize {
    linear_congruential_generator(2_usize.pow(31), 1103515245, 12345, seed)
}

pub fn seeded_prng(seed: u64) -> impl FnMut() -> u64 {
    let mut state = seed;
    let a = 1103515245;
    let c = 12345;
    let m = 2u64.pow(32);
    move || {
        state = (a * state + c) % m;
        state
    }
}

pub trait Pickable<T> {
    fn pick_lcg(&self, seed: usize) -> &T;
    fn pick_lcg2(&self, generator: impl FnMut() -> u64) -> &T;
}
impl<T> Pickable<T> for Vec<T> {
    fn pick_lcg(&self, seed: usize) -> &T {
        let max = self.len() as usize;
        let rnd = lcg_sane(seed);

        let index: usize = rnd % max;
        return &self[index];
    }

    fn pick_lcg2(&self, generator: impl FnMut() -> u64) -> &T {
        let max = self.len() as u64;
        let rnd = generator();

        let index: u64 = rnd % max;
        return &self[index as usize];
    }
}
impl<T, const SIZE: usize> Pickable<T> for [T; SIZE] {
    fn pick_lcg(&self, seed: usize) -> &T {
        let max = self.len() as usize;
        let rnd = lcg_sane(seed);

        let index: usize = rnd % max;
        return &self[index];
    }
    fn pick_lcg2(&self, generator: impl FnMut() -> u64) -> &T {
        let max = self.len() as u64;
        let rnd = generator();

        let index: u64 = rnd % max;
        return &self[index as usize];
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RandAlgo {
    LCG,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mwc() {
        let rnd = super::multiple_with_carry(1);
        assert_eq!(rnd, 1.0);
    }
}
