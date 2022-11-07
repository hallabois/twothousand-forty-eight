// https://stackoverflow.com/questions/3062746/special-simple-random-number-generator

// https://en.wikipedia.org/wiki/Linear_congruential_generator
pub fn linear_congruential_generator(m: usize, a: usize, c: usize, seed: usize) -> usize {
    assert_ne!(seed, 0, "A seed of 0 will always return 0");
    return (a * seed + c) % m;
}

pub fn lcg_sane(seed: usize) -> usize {
    linear_congruential_generator(2_usize.pow(31), 1103515245, 12345, seed)
}

pub trait Pickable<T> {
    fn pick_lcg(&self, seed: usize) -> &T;
}
impl<T> Pickable<T> for Vec<T> {
    fn pick_lcg(&self, seed: usize) -> &T {
        let max = self.len() as usize;
        let rnd = lcg_sane(seed);

        let index: usize = rnd % max;
        return &self[index];
    }
}
impl<T, const SIZE: usize> Pickable<T> for [T; SIZE] {
    fn pick_lcg(&self, seed: usize) -> &T {
        let max = self.len() as usize;
        let rnd = lcg_sane(seed);

        let index: usize = rnd % max;
        return &self[index];
    }
}
