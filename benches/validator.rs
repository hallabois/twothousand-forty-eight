use criterion::{criterion_group, criterion_main, Criterion};

use twothousand_forty_eight::parser;
use twothousand_forty_eight::recording::Recording;
use twothousand_forty_eight::validator;

pub const GAMEOBSCENE: &str = include_str!("../games/GAME4X4BIG.txt");

fn bvalidate_first_move(history: Recording) {
    validator::validate_first_move(&history);
}

fn bvalidate_history(history: Recording) {
    validator::validate_history(history).unwrap();
}

fn validate_history(c: &mut Criterion) {
    let history = parser::parse_data(String::from(GAMEOBSCENE)).unwrap();
    c.bench_function("validate_first_move", |b| {
        b.iter(|| bvalidate_history(history.clone()))
    });
}

fn validate_first_move(c: &mut Criterion) {
    let history = parser::parse_data(String::from(GAMEOBSCENE)).unwrap();
    c.bench_function("validate_first_move", |b| {
        b.iter(|| bvalidate_first_move(history.clone()))
    });
}

criterion_group!(benches, validate_first_move, validate_history);
criterion_main!(benches);
