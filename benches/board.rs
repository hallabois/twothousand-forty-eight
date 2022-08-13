use criterion::{criterion_group, criterion_main, Criterion};

use twothousand_forty_eight::{board, add_random_to_board};

fn is_move_possible(c: &mut Criterion) {
    c.bench_function("board is_move_possible", |b| b.iter(|| { 
            let mut game = board::Board::new();
            add_random_to_board(&mut game);
            add_random_to_board(&mut game);
            board::is_move_possible(game, twothousand_forty_eight::direction::Direction::UP);
            board::is_move_possible(game, twothousand_forty_eight::direction::Direction::RIGHT);
            board::is_move_possible(game, twothousand_forty_eight::direction::Direction::DOWN);
            board::is_move_possible(game, twothousand_forty_eight::direction::Direction::LEFT);
        }
    ));
}

criterion_group!(benches, 
	is_move_possible
);
criterion_main!(benches);