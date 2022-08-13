use criterion::{criterion_group, criterion_main, Criterion};

use twothousand_forty_eight::parser;

pub const GAME3X3: &str = include_str!("../games/GAME3X3A.txt");
pub const GAME3X3B: &str = include_str!("../games/GAME3X3B.txt");
pub const GAME4X4: &str = include_str!("../games/GAME4X4.txt");
pub const GAME4X4B: &str = include_str!("../games/GAME4X4B.txt");
pub const GAME4X4C: &str = include_str!("../games/GAME4X4C.txt");
pub const GAMEOBSCENE: &str = include_str!("../games/GAME4X4BIG.txt");

fn parse_a(c: &mut Criterion) {
    c.bench_function("parser GAME3X3", |b| b.iter(|| 
    	parser::parse_data(String::from(GAME3X3)).unwrap()
    ));
}

fn parse_b(c: &mut Criterion) {
    c.bench_function("parser GAME3X3B", |b| b.iter(|| 
    	parser::parse_data(String::from(GAME3X3B)).unwrap()
    ));
}

fn parse_c(c: &mut Criterion) {
    c.bench_function("parser GAME4X4", |b| b.iter(|| 
    	parser::parse_data(String::from(GAME4X4)).unwrap()
    ));
}

fn parse_d(c: &mut Criterion) {
    c.bench_function("parser GAME4X4B", |b| b.iter(|| 
    	parser::parse_data(String::from(GAME4X4B)).unwrap()
    ));
}

fn parse_e(c: &mut Criterion) {
    c.bench_function("parser GAME4X4C", |b| b.iter(|| 
    	parser::parse_data(String::from(GAME4X4C)).unwrap()
    ));
}

fn parse_f(c: &mut Criterion) {
    c.bench_function("validate_first_move", |b| b.iter(|| 
    	parser::parse_data(String::from(GAMEOBSCENE)).unwrap()
    ));
}

criterion_group!(benches, 
	parse_a,
	parse_b,
	parse_c,
	parse_d,
	parse_e,
	parse_f
);
criterion_main!(benches);