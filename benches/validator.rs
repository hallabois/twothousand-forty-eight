use criterion::{criterion_group, criterion_main, Criterion};

use twothousand_forty_eight::v1::parser;
use twothousand_forty_eight::v1::recording::Recording;
use twothousand_forty_eight::v1::validator;

pub const GAMEOBSCENE: &str = include_str!("../games/v1/GAME4X4BIG.txt");
pub const GAMELIST: &str = include_str!("../games/v1/composite.txt");
pub fn get_random_game() -> &'static str {
    let mut rng = rand::thread_rng();
    let games: Vec<&str> = GAMELIST.split('\n').collect();
    let index = rand::Rng::gen_range(&mut rng, 0..games.len());
    games[index]
}

fn bench_function(c: &mut Criterion, id: &str, f: Box<dyn Fn(&mut criterion::Bencher)>) {
    let mut group = c.benchmark_group("validator");
    group.sampling_mode(criterion::SamplingMode::Auto);
    group.bench_function(id, f);
    group.finish();
}

fn bvalidate_first_move(history: Recording) {
    validator::validate_first_move(&history);
}

fn bvalidate_history(history: Recording) {
    validator::validate_history(history).unwrap();
}

fn validate_history(c: &mut Criterion) {
    let history = parser::parse_data(GAMEOBSCENE).unwrap();
    bench_function(
        c,
        "validate_history",
        Box::new(move |b| b.iter(|| bvalidate_history(history.clone()))),
    );
}

fn validate_history_random(c: &mut Criterion) {
    let game = get_random_game();
    let history = parser::parse_data(game).unwrap();
    bench_function(
        c,
        "validate_history random",
        Box::new(move |b| b.iter(|| bvalidate_history(history.clone()))),
    );
}

fn validate_first_move(c: &mut Criterion) {
    let history = parser::parse_data(GAMEOBSCENE).unwrap();
    bench_function(
        c,
        "validate_first_move",
        Box::new(move |b| b.iter(|| bvalidate_first_move(history.clone()))),
    );
}

fn validate_first_move_random(c: &mut Criterion) {
    let game = get_random_game();
    let history = parser::parse_data(game).unwrap();
    bench_function(
        c,
        "validate_first_move random",
        Box::new(move |b| b.iter(|| bvalidate_first_move(history.clone()))),
    );
}

criterion_group!(
    benches,
    validate_first_move,
    validate_first_move_random,
    validate_history,
    validate_history_random
);
criterion_main!(benches);
