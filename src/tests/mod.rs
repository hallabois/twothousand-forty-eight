
mod lib_testgames;

#[cfg(test)]
mod board {
    #[test]
    fn creation_works(){
        use crate::board;
        use board::Board;

        for w in 0..board::MAX_WIDTH {
            for h in 0..board::MAX_HEIGHT {
                let mut board = Board{
                    width: w,
                    height: h,
                    tiles: board::create_tiles(w, h)
                };

                let mut index = 0;
                for x in 0..w {
                    for y in 0..h {
                        board.set_tile(x, y, index);
                        index += 1;
                    }
                }

                println!("w:{} h:{}", w, h);
                board::print_board(board.tiles, w, h);

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
}

#[cfg(test)]
mod parser {
    use super::lib_testgames;
    use crate::parser;

    #[test]
    fn works_4x4(){
        use lib_testgames::GAME4X4;
        let history4x4 = parser::parse_data(String::from(GAME4X4)).unwrap();
        assert_eq!(history4x4.width, 4);
        assert_eq!(history4x4.height, 4);
        assert_eq!(history4x4.history.len(), 576);
    }

    #[test]
    fn works_3x3(){
        use lib_testgames::GAME3X3;
        let history4x4 = parser::parse_data(String::from(GAME3X3)).unwrap();
        assert_eq!(history4x4.width, 3);
        assert_eq!(history4x4.height, 3);
        assert_eq!(history4x4.history.len(), 500);
    }
}

#[cfg(test)]
mod validator {
    use super::lib_testgames;
    use crate::parser;
    use crate::validator;

    pub fn assert_score(score: usize, expected: usize, score_margin: usize) {
        assert!(score <= expected+score_margin, "{} !< {}", score, expected+score_margin);
    }

    #[test]
    fn works_normal_4x4_0breaks_a() {
        use lib_testgames::GAME4X4B;
        let history = parser::parse_data(String::from(GAME4X4B)).unwrap();
        let first_move_valid = validator::validate_first_move(&history);
        assert_eq!(first_move_valid, true);
        let (result1, score, score_margin, breaks) = validator::validate_history(history);
        assert_eq!(result1, true);
        assert_score(score, 2788, score_margin);
        assert_eq!(breaks, 0);
    }
    #[test]
    fn works_normal_4x4_0breaks_b() {
        use lib_testgames::GAME4X4C;
        let history = parser::parse_data(String::from(GAME4X4C)).unwrap();
        let first_move_valid = validator::validate_first_move(&history);
        assert_eq!(first_move_valid, true);
        let (result1, score, score_margin, breaks) = validator::validate_history(history);
        assert_eq!(result1, true);
        assert_score(score, 2624, score_margin);
        assert_eq!(breaks, 0);
    }
    #[test]
    fn works_normal_4x4_2breaks() {
        use lib_testgames::GAME4X4;
        let history = parser::parse_data(String::from(GAME4X4)).unwrap();
        let first_move_valid = validator::validate_first_move(&history);
        assert_eq!(first_move_valid, true);
        let (result1, score, score_margin, breaks) = validator::validate_history(history);
        assert_eq!(result1, true);
        assert_score(score, 6048, score_margin);
        assert_eq!(breaks, 2);
    }
    #[test]
    fn works_normal_3x3_0breaks_a() {
        use lib_testgames::GAME3X3;
        let history = parser::parse_data(String::from(GAME3X3)).unwrap();
        let first_move_valid = validator::validate_first_move(&history);
        assert_eq!(first_move_valid, true);
        let (result1, score, score_margin, breaks) = validator::validate_history(history);
        assert_eq!(result1, true);
        assert_score(score, 14220, score_margin);
        assert_eq!(breaks, 0);
    }
    #[test]
    fn works_normal_3x3_0breaks_b() {
        use lib_testgames::GAME3X3B;
        let history = parser::parse_data(String::from(GAME3X3B)).unwrap();
        let first_move_valid = validator::validate_first_move(&history);
        assert_eq!(first_move_valid, true);
        let (result1, score, score_margin, breaks) = validator::validate_history(history);
        assert_eq!(result1, true);
        assert_score(score, 208, score_margin);
        assert_eq!(breaks, 0);
    }
    #[test]
    fn works_looong_4x4_0breaks() {
        use lib_testgames::GAMEOBSCENE;
        let history = parser::parse_data(String::from(GAMEOBSCENE)).unwrap();
        let first_move_valid = validator::validate_first_move(&history);
        assert_eq!(first_move_valid, true);
        let (result1, score, score_margin, breaks) = validator::validate_history(history);
        assert_eq!(result1, true);
        assert_score(score, 200028, score_margin);
        assert_eq!(breaks, 0);
    }
}

#[cfg(test)]
#[cfg(feature = "history_hash")]
mod history_hash {
    use super::lib_testgames;
    use crate::parser;

    #[test]
    fn history_hash_works() {
        use lib_testgames::GAME4X4;
        let history = parser::parse_data(String::from(GAME4X4)).unwrap();
        assert_eq!(history.hash_v1(), String::from("9CAC2643E4E5F66E18FD9150320471F016CAF69FA3865A6DAE1DD9726F6792F5"));
    }
}

#[cfg(test)]
#[cfg(feature = "tile_merged_from")]
mod tile_merged_from {
    use crate::board;
    use board::Board;
    // use board::tile::Tile;

    #[test]
    fn tile_merged_from_works_4x4() {
        let mut game = Board::new();
        game.set_tile(0, 0, 4);
        let t1 = game.tiles[0][0].unwrap();
        game.set_tile(1, 0, 4);
        let t2 = game.tiles[0][1].unwrap();
        println!("Starting board:");
        board::print_board(game.tiles, 4, 4);

        let (new_tiles, possible, _scoregain) = board::is_move_possible(game, crate::direction::Direction::LEFT);
        assert!(possible);

        println!("Board on next move:");
        board::print_board(new_tiles, 4, 4);

        let nt = new_tiles[0][0].unwrap();

        assert_ne!(t1.id, nt.id);
        assert_ne!(t2.id, nt.id);
        println!("nt merged from: {:?}", nt.merged_from);
        assert!(nt.merged_from == Some([t1.id, t2.id]) || nt.merged_from == Some([t2.id, t1.id]));

        game.tiles = new_tiles;
        let (new_tiles, possible, _scoregain) = board::is_move_possible(game, crate::direction::Direction::RIGHT);
        assert!(possible);

        println!("Board on next move:");
        board::print_board(new_tiles, 4, 4);

        let nt = new_tiles[0][0].unwrap();

        assert_ne!(t1.id, nt.id);
        assert_ne!(t2.id, nt.id);
        println!("nt merged from: {:?}", nt.merged_from);
        assert!(nt.merged_from.is_none());
    }
}
