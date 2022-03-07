
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
        let history4x4 = parser::parse_data(String::from(GAME4X4));
        assert_eq!(history4x4.width, 4);
        assert_eq!(history4x4.height, 4);
        assert_eq!(history4x4.history.len(), 576);
    }

    #[test]
    fn works_3x3(){
        use lib_testgames::GAME3X3;
        let history4x4 = parser::parse_data(String::from(GAME3X3));
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
        let history = parser::parse_data(String::from(GAME4X4B));
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
        let history = parser::parse_data(String::from(GAME4X4C));
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
        let history = parser::parse_data(String::from(GAME4X4));
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
        let history = parser::parse_data(String::from(GAME3X3));
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
        let history = parser::parse_data(String::from(GAME3X3B));
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
        let history = parser::parse_data(String::from(GAMEOBSCENE));
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
        let history = parser::parse_data(String::from(GAME4X4));
        assert_eq!(history.hash_v1(), String::from("9CAC2643E4E5F66E18FD9150320471F016CAF69FA3865A6DAE1DD9726F6792F5"));
    }
}
