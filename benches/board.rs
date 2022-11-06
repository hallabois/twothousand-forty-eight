use criterion::{criterion_group, criterion_main, Criterion};

use twothousand_forty_eight::{add_random_to_board, board};

fn is_move_possible(c: &mut Criterion) {
    c.bench_function("board is_move_possible", |b| {
        b.iter(|| {
            let mut game = board::Board::new();
            add_random_to_board(&mut game);
            add_random_to_board(&mut game);
            board::check_move(game, twothousand_forty_eight::direction::Direction::UP);
            board::check_move(game, twothousand_forty_eight::direction::Direction::RIGHT);
            board::check_move(game, twothousand_forty_eight::direction::Direction::DOWN);
            board::check_move(game, twothousand_forty_eight::direction::Direction::LEFT);
        })
    });
}

criterion_group!(benches, is_move_possible);
criterion_main!(benches);
