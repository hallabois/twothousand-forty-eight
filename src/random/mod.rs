//! Pseudo-random number generation for game logic

// https://stackoverflow.com/questions/3062746/special-simple-random-number-generator

use std::num::Wrapping;

use serde::{Deserialize, Serialize};

// https://en.wikipedia.org/wiki/Linear_congruential_generator
pub fn linear_congruential_generator(m: u32, a: u32, c: u32, seed: &mut u32) -> u32 {
    // Wrapping is used to allow overflow
    let output = (Wrapping(a) * Wrapping(*seed) + Wrapping(c)).0 % m;
    *seed = output;
    output
}

pub fn lcg_sane(seed: &mut u32) -> u32 {
    linear_congruential_generator(2147483647, 1103515245, 12345, seed)
}

pub trait Pickable<T> {
    fn pick_lcg(&self, seed: &mut u32) -> &T;
}
impl<T> Pickable<T> for Vec<T> {
    fn pick_lcg(&self, seed: &mut u32) -> &T {
        let max = self.len() as u32;
        let rnd = lcg_sane(seed);

        let index: u32 = rnd % max;
        &self[index as usize]
    }
}
impl<T, const SIZE: usize> Pickable<T> for [T; SIZE] {
    fn pick_lcg(&self, seed: &mut u32) -> &T {
        let max = self.len() as u32;
        let rnd = lcg_sane(seed);

        let index: u32 = rnd % max;
        &self[index as usize]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RandAlgo {
    LCG = 0,
}
