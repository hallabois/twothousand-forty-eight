use criterion::{criterion_group, criterion_main, Criterion};

use twothousand_forty_eight::parser;

pub const GAME3X3: &str = include_str!("../games/v1/GAME3X3A.txt");
pub const GAME3X3B: &str = include_str!("../games/v1/GAME3X3B.txt");
pub const GAME4X4: &str = include_str!("../games/v1/GAME4X4.txt");
pub const GAME4X4B: &str = include_str!("../games/v1/GAME4X4B.txt");
pub const GAME4X4C: &str = include_str!("../games/v1/GAME4X4C.txt");
pub const GAMEOBSCENE: &str = include_str!("../games/v1/GAME4X4BIG.txt");
pub const GAMELIST: &str = include_str!("../games/v1/composite.txt");
pub fn get_random_game() -> &'static str {
    let mut rng = rand::thread_rng();
    let games: Vec<&str> = GAMELIST.split("\n").collect();
    let index = rand::Rng::gen_range(&mut rng, 0..games.len());
    games[index]
}

fn bench_function_flat(c: &mut Criterion, id: &str, f: Box<dyn Fn(&mut criterion::Bencher)>) {
    let mut group = c.benchmark_group("parser");
    group.sampling_mode(criterion::SamplingMode::Flat);
    group.sample_size(80);
    group.bench_function(id, f);
    group.finish();
}

fn parse_random(c: &mut Criterion) {
    let game = get_random_game();
    bench_function_flat(
        c,
        "random",
        Box::new(|bencher| bencher.iter(|| parser::parse_data(game).unwrap())),
    );
}

fn parse_a(c: &mut Criterion) {
    bench_function_flat(
        c,
        "GAME3X3",
        Box::new(|bencher| bencher.iter(|| parser::parse_data(GAME3X3).unwrap())),
    );
}

fn parse_b(c: &mut Criterion) {
    bench_function_flat(
        c,
        "GAME3X3B",
        Box::new(|bencher| bencher.iter(|| parser::parse_data(GAME3X3B).unwrap())),
    );
}

fn parse_c(c: &mut Criterion) {
    bench_function_flat(
        c,
        "GAME4X4",
        Box::new(|bencher| bencher.iter(|| parser::parse_data(GAME4X4).unwrap())),
    );
}

fn parse_d(c: &mut Criterion) {
    bench_function_flat(
        c,
        "GAME4X4B",
        Box::new(|bencher| bencher.iter(|| parser::parse_data(GAME4X4B).unwrap())),
    );
}

fn parse_e(c: &mut Criterion) {
    bench_function_flat(
        c,
        "GAME4X4C",
        Box::new(|bencher| bencher.iter(|| parser::parse_data(GAME4X4C).unwrap())),
    );
}

fn parse_f(c: &mut Criterion) {
    bench_function_flat(
        c,
        "GAME4X4OBSCENE",
        Box::new(|bencher| bencher.iter(|| parser::parse_data(GAMEOBSCENE).unwrap())),
    );
}

criterion_group!(
    benches,
    parse_random,
    parse_a,
    parse_b,
    parse_c,
    parse_d,
    parse_e,
    parse_f
);
criterion_main!(benches);
