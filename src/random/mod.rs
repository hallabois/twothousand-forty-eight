//! Pseudo-random number generation for game logic

// https://stackoverflow.com/questions/3062746/special-simple-random-number-generator

use std::num::Wrapping;

use serde::{Deserialize, Serialize};

// https://en.wikipedia.org/wiki/Linear_congruential_generator
pub fn linear_congruential_generator(m: usize, a: usize, c: usize, seed: &mut usize) -> usize {
    // Wrapping is used to allow overflow
    let output = (Wrapping(a) * Wrapping(*seed) + Wrapping(c)).0 % m;
    *seed = output;
    output
}

pub fn lcg_sane(seed: &mut usize) -> usize {
    linear_congruential_generator(
        2147483647, // 2^31 - 1
        1103515245, // 2^31 - 1
        12345,      // 2^31 - 1
        seed,
    )
}

pub trait Pickable<T> {
    fn pick_lcg(&self, seed: &mut usize) -> &T;
}
impl<T> Pickable<T> for Vec<T> {
    fn pick_lcg(&self, seed: &mut usize) -> &T {
        let max = self.len();
        let rnd = lcg_sane(seed);

        let index: usize = rnd % max;
        &self[index]
    }
}
impl<T, const SIZE: usize> Pickable<T> for [T; SIZE] {
    fn pick_lcg(&self, seed: &mut usize) -> &T {
        let max = self.len();
        let rnd = lcg_sane(seed);

        let index: usize = rnd % max;
        &self[index]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RandAlgo {
    LCG = 0,
}
