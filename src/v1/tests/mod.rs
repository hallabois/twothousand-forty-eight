pub mod lib_testgames;

#[cfg(test)]
mod board {
    use crate::board;
    use board::Board;
    #[test]
    fn creation_works() {
        for w in 0..board::MAX_WIDTH {
            for h in 0..board::MAX_HEIGHT {
                let mut board = Board::new(w, h, 1);

                let mut index = 0;
                for x in 0..w {
                    for y in 0..h {
                        board.set_tile(x, y, index);
                        index += 1;
                    }
                }

                println!("w:{} h:{}", w, h);
                println!("{}", board);

                index = 0;
                for x in 0..w {
                    for y in 0..h {
                        assert_eq!(board.tiles[y][x].unwrap().value, index);
                        index += 1;
                    }
                }
            }
        }
    }

    #[test]
    fn has_possible_moves_a() {
        let mut board_a = Board::default();
        board_a.set_tile(0, 0, 2);
        assert!(board_a.has_possible_moves());
    }

    #[test]
    fn get_total_value() {
        let mut board_a = Board::default();
        board_a.set_tile(0, 0, 2);
        board_a.set_tile(0, 1, 4);
        board_a.set_tile(3, 1, 2);
        let value = board_a.get_total_value();
        assert_eq!(value, 8);
    }

    #[test]
    fn get_id_sum() {
        let mut board = Board::new(4, 4, 1);
        board.set_tile(0, 0, 2);
        board.set_tile(0, 1, 4);
        board.set_tile(3, 1, 2);
        let sum = board.get_id_sum();
        assert_eq!(sum, 157);
    }
}

#[cfg(test)]
mod parser {
    use std::collections::HashSet;

    use rayon::prelude::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

    use super::lib_testgames;
    use crate::{
        board::tile::Tile,
        v1::{parser, recording::Recording},
    };

    fn check_history_ids(history: Recording) {
        for (tiles, _direction, _addition) in history.history {
            let mut seen_ids: HashSet<usize> = HashSet::new();
            for y in 0..history.height {
                for x in 0..history.width {
                    if let Some(tile) = tiles[y][x] {
                        if seen_ids.contains(&tile.id) {
                            panic!("board contains multiple tiles with the same id");
                        } else {
                            seen_ids.insert(tile.id);
                        }
                    }
                }
            }
        }
    }

    #[test]
    #[should_panic]
    fn id_test_works() {
        let mut history = parser::parse_data("2x1S2.0;0:2.0;0").unwrap();
        let tile1 = history.history[0].0[0][0].unwrap();
        let tile2 = history.history[0].0[1][0].unwrap();
        history.history[0].0[0][0] = Some(Tile { id: 0, ..tile1 });
        history.history[0].0[1][0] = Some(Tile { id: 0, ..tile2 });
        check_history_ids(history);
    }

    #[test]
    fn works_4x4() {
        use lib_testgames::GAME4X4;
        let history4x4 = parser::parse_data(GAME4X4).unwrap();
        assert_eq!(history4x4.width, 4);
        assert_eq!(history4x4.height, 4);
        assert_eq!(history4x4.history.len(), 576);
    }

    #[test]
    fn works_3x3() {
        use lib_testgames::GAME3X3;
        let history = parser::parse_data(GAME3X3).unwrap();
        assert_eq!(history.width, 3);
        assert_eq!(history.height, 3);
        assert_eq!(history.history.len(), 500);
    }

    #[test]
    fn produces_coherrent_ids_a() {
        use lib_testgames::GAME4X4;
        let history = parser::parse_data(GAME4X4).unwrap();
        check_history_ids(history);
    }

    #[test]
    fn produces_coherrent_ids_b() {
        use lib_testgames::GAME3X3;
        let history = parser::parse_data(GAME3X3).unwrap();
        check_history_ids(history);
    }

    #[test]
    #[ignore = "slow"]
    /// Test about 10 000 games gathered from players
    fn works_all_real() {
        use lib_testgames::GAMELIST;
        let games: Vec<&str> = GAMELIST.split('\n').collect();
        games.par_iter().enumerate().for_each(|(i, game)| {
            println!("parsing game {} / {}", i, games.len());
            let history = parser::parse_data(game).unwrap();
            assert!(history.width > 0);
            assert!(history.height > 0);
            assert!(history.history.len() > 0);
        });
    }

    #[test]
    #[ignore = "slow"]
    fn produces_coherrent_ids_all() {
        use lib_testgames::GAMELIST;
        let games: Vec<&str> = GAMELIST.split("\n").collect();
        games.par_iter().enumerate().for_each(|(i, game)| {
            println!("parsing game {} / {}", i, games.len());
            let history = parser::parse_data(game).unwrap();
            check_history_ids(history);
        });
    }
}

#[cfg(test)]
mod validator {
    use rayon::prelude::IndexedParallelIterator;
    use rayon::prelude::IntoParallelRefIterator;
    use rayon::prelude::ParallelIterator;

    use super::lib_testgames;
    use crate::board::Board;
    use crate::v1::parser;
    use crate::v1::validator;

    pub fn assert_score(score: usize, expected: usize, score_margin: usize) {
        assert!(
            score <= expected + score_margin,
            "{} !< {}",
            score,
            expected + score_margin
        );
    }

    #[test]
    fn history_reconstruction() {
        use lib_testgames::GAME4X4;
        let recording = parser::parse_data(GAME4X4).unwrap();
        let history = recording.history.clone();
        let reconstruction = validator::reconstruct_history(recording.clone()).unwrap();

        assert_eq!(history.len(), reconstruction.history.len());
        let mut rng_state = 0;

        for (i, item) in history.iter().enumerate() {
            println!("history index {}", i);
            let history_tiles = item.0;
            let history_board = Board::from((history_tiles, rng_state));
            println!("recorded board");
            println!("{}", Board::from((history_tiles, 0)));
            let rec_board = reconstruction.history[i];
            println!("predicted board");
            println!("{}", Board::from((rec_board.tiles, 0)));

            let t1 = history_board.get_all_tiles();
            let t2 = rec_board.get_all_tiles();
            assert_eq!(t1.len(), t2.len());
            for ti in 0..(t1.len() - 1) {
                println!("tile index {}", ti);
                let ta = t1[ti];
                let tb = t2[ti];
                assert_eq!(ta.x, tb.x);
                assert_eq!(ta.y, tb.y);
                assert_eq!(ta.value, tb.value);
            }
            rng_state = rec_board.rng_state;
        }
    }

    #[test]
    fn works_normal_4x4_0breaks_a() {
        use lib_testgames::GAME4X4B;
        let history = parser::parse_data(GAME4X4B).unwrap();
        let first_move_valid = validator::validate_first_move(&history);
        assert!(first_move_valid);
        let result = validator::validate_history(history).unwrap();
        assert_score(result.score, 2788, result.score_margin);
        assert_eq!(result.breaks, 0);
    }
    #[test]
    fn works_normal_4x4_0breaks_b() {
        use lib_testgames::GAME4X4C;
        let history = parser::parse_data(GAME4X4C).unwrap();
        let first_move_valid = validator::validate_first_move(&history);
        assert!(first_move_valid);
        let result = validator::validate_history(history).unwrap();
        assert_score(result.score, 2624, result.score_margin);
        assert_eq!(result.breaks, 0);
    }
    #[test]
    fn works_normal_4x4_2breaks() {
        use lib_testgames::GAME4X4;
        let history = parser::parse_data(GAME4X4).unwrap();
        let first_move_valid = validator::validate_first_move(&history);
        assert!(first_move_valid);
        let result = validator::validate_history(history).unwrap();
        assert_score(result.score, 6048, result.score_margin);
        assert_eq!(result.breaks, 2);
    }
    #[test]
    fn works_normal_3x3_0breaks_a() {
        use lib_testgames::GAME3X3;
        let history = parser::parse_data(GAME3X3).unwrap();
        let first_move_valid = validator::validate_first_move(&history);
        assert!(first_move_valid);
        let result = validator::validate_history(history).unwrap();
        assert_score(result.score, 14220, result.score_margin);
        assert_eq!(result.breaks, 0);
    }
    #[test]
    fn works_normal_3x3_0breaks_b() {
        use lib_testgames::GAME3X3B;
        let history = parser::parse_data(GAME3X3B).unwrap();
        let first_move_valid = validator::validate_first_move(&history);
        assert!(first_move_valid);
        let result = validator::validate_history(history).unwrap();
        assert_score(result.score, 208, result.score_margin);
        assert_eq!(result.breaks, 0);
    }
    #[test]
    fn works_looong_4x4_0breaks() {
        use lib_testgames::GAMEOBSCENE;
        let history = parser::parse_data(GAMEOBSCENE).unwrap();
        let first_move_valid = validator::validate_first_move(&history);
        assert!(first_move_valid);
        let result = validator::validate_history(history).unwrap();
        assert_score(result.score, 200028, result.score_margin);
        assert_eq!(result.breaks, 0);
    }

    #[test]
    #[ignore = "slow"]
    /// Test about 10 000 games gathered from players
    fn works_all_real() {
        use lib_testgames::GAMELIST;
        let games: Vec<&str> = GAMELIST.split('\n').collect();
        games.par_iter().enumerate().for_each(|(i, game)| {
            println!("parsing game {} / {}", i, games.len());
            let history = parser::parse_data(game).unwrap();
            let first_move_valid = validator::validate_first_move(&history);
            assert!(first_move_valid);
            let result = validator::validate_history(history).unwrap();
            assert!(result.valid);
        });
    }
}

#[cfg(test)]
mod serializers {
    use crate::board::tile::Tile;

    #[test]
    fn tile_serializer_null() {
        let t = Tile::new(0, 0, 0, crate::board::tile::InitialID::Id(0));
        assert_eq!(t.to_json(), "null");
    }

    #[test]
    fn tile_serializer_some() {
        let t = Tile::new(0, 1, 4, crate::board::tile::InitialID::Id(0));

        assert_eq!(
            t.to_json(),
            "{\"x\":0,\"y\":1,\"value\":4,\"id\":0,\"merged_from\":null}"
        );
    }
}

#[cfg(test)]
mod history_hash {
    use super::lib_testgames;
    use crate::v1::parser;

    #[test]
    fn history_hash_works() {
        use lib_testgames::GAME4X4;
        let history = parser::parse_data(GAME4X4).unwrap();
        assert_eq!(
            history.hash_v1(),
            String::from("9CAC2643E4E5F66E18FD9150320471F016CAF69FA3865A6DAE1DD9726F6792F5")
        );
    }
}

#[cfg(test)]
mod tile_merged_from {
    use crate::board::{self, MoveResult};
    use board::Board;

    #[test]
    fn tile_merged_from_works_4x4() {
        let mut game = Board::default();
        game.set_tile(0, 0, 4);
        let t1 = game.tiles[0][0].unwrap();
        game.set_tile(1, 0, 4);
        let t2 = game.tiles[0][1].unwrap();
        println!("Starting board:");
        println!("{}", game);

        let MoveResult {
            board,
            score_gain: _,
        } = board::check_move(game, crate::direction::Direction::LEFT).unwrap();

        println!("Board on next move:");
        println!("{}", board);

        let nt = board.tiles[0][0].unwrap();

        assert_ne!(t1.id, nt.id);
        assert_ne!(t2.id, nt.id);
        println!("nt merged from: {:?}", nt.merged_from);
        assert!(nt.merged_from == Some([t1.id, t2.id]) || nt.merged_from == Some([t2.id, t1.id]));

        game = board;
        let MoveResult {
            board,
            score_gain: _,
        } = board::check_move(game, crate::direction::Direction::RIGHT).unwrap();

        println!("Board on next move:");
        println!("{}", board);

        let nt = board.tiles[0][0].unwrap();

        assert_ne!(t1.id, nt.id);
        assert_ne!(t2.id, nt.id);
        println!("nt merged from: {:?}", nt.merged_from);
        assert!(nt.merged_from.is_none());
    }
}
