use criterion::{criterion_group, criterion_main, Criterion};

use twothousand_forty_eight::{add_random_to_board, board};

fn bench_function(c: &mut Criterion, id: &str, f: Box<dyn Fn(&mut criterion::Bencher)>) {
    let mut group = c.benchmark_group("board");
    group.sampling_mode(criterion::SamplingMode::Auto);
    group.bench_function(id, f);
    group.finish();
}

fn check_move_two(c: &mut Criterion) {
    bench_function(
        c,
        "check_move 2",
        Box::new(|bencher| {
            bencher.iter(|| {
                let mut game = board::Board::default();
                add_random_to_board(&mut game);
                add_random_to_board(&mut game);
                let _ = board::check_move(game, twothousand_forty_eight::direction::Direction::UP);
                let _ =
                    board::check_move(game, twothousand_forty_eight::direction::Direction::RIGHT);
                let _ =
                    board::check_move(game, twothousand_forty_eight::direction::Direction::DOWN);
                let _ =
                    board::check_move(game, twothousand_forty_eight::direction::Direction::LEFT);
            })
        }),
    );
}

fn check_move_four(c: &mut Criterion) {
    bench_function(
        c,
        "check_move 4",
        Box::new(|bencher| {
            bencher.iter(|| {
                let mut game = board::Board::default();
                add_random_to_board(&mut game);
                add_random_to_board(&mut game);
                add_random_to_board(&mut game);
                add_random_to_board(&mut game);
                let _ = board::check_move(game, twothousand_forty_eight::direction::Direction::UP);
                let _ =
                    board::check_move(game, twothousand_forty_eight::direction::Direction::RIGHT);
                let _ =
                    board::check_move(game, twothousand_forty_eight::direction::Direction::DOWN);
                let _ =
                    board::check_move(game, twothousand_forty_eight::direction::Direction::LEFT);
            })
        }),
    );
}

criterion_group!(benches, check_move_two, check_move_four);
criterion_main!(benches);
