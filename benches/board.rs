use criterion::{criterion_group, criterion_main, Criterion};

use twothousand_forty_eight::{add_random_to_board, board};

fn check_move(c: &mut Criterion) {
    c.bench_function("board check_move", |b| {
        b.iter(|| {
            let mut game = board::Board::default();
            add_random_to_board(&mut game, None);
            add_random_to_board(&mut game, None);
            board::check_move(game, twothousand_forty_eight::direction::Direction::UP);
            board::check_move(game, twothousand_forty_eight::direction::Direction::RIGHT);
            board::check_move(game, twothousand_forty_eight::direction::Direction::DOWN);
            board::check_move(game, twothousand_forty_eight::direction::Direction::LEFT);
        })
    });
}

criterion_group!(benches, check_move);
criterion_main!(benches);
