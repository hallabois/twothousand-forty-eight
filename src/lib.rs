pub mod board;
pub mod parser;
pub mod validator;
pub mod direction;
pub mod recording;

pub const DEBUG_INFO: bool = false;

pub mod lib_testgames;

#[cfg(test)]
mod tests {
    #[test]
    fn board_creation_works(){
        use super::board::Board;

        for w in 0..super::board::MAX_WIDTH {
            for h in 0..super::board::MAX_HEIGHT {
                let mut board = Board{
                    width: w,
                    height: h,
                    tiles: super::board::create_tiles(w, h)
                };

                let mut index = 0;
                for x in 0..w {
                    for y in 0..h {
                        board.set_tile(x, y, index);
                        index += 1;
                    }
                }

                println!("w:{} h:{}", w, h);
                super::board::print_board(board.tiles, w, h);

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
    fn parser_works_4x4(){
        use super::lib_testgames::GAME4X4;
        let history4x4 = super::parser::parse_data(String::from(GAME4X4));
        assert_eq!(history4x4.width, 4);
        assert_eq!(history4x4.height, 4);
        assert_eq!(history4x4.history.len(), 576);
    }
    #[test]
    fn parser_works_3x3(){
        use super::lib_testgames::GAME3X3;
        let history4x4 = super::parser::parse_data(String::from(GAME3X3));
        assert_eq!(history4x4.width, 3);
        assert_eq!(history4x4.height, 3);
        assert_eq!(history4x4.history.len(), 500);
    }
    #[test]
    fn validator_works_normal_4x4_2breaks() {
        use super::lib_testgames::GAME4X4;
        let history = super::parser::parse_data(String::from(GAME4X4));
        let first_move_valid = super::validator::validate_first_move(&history);
        assert_eq!(first_move_valid, true);
        let (result1, score, breaks) = super::validator::validate_history(history);
        assert_eq!(result1, true);
        assert_eq!(score, 6052);
        assert_eq!(breaks, 2);
    }
    #[test]
    fn validator_works_normal_3x3_0breaks() {
        use super::lib_testgames::GAME3X3;
        let history = super::parser::parse_data(String::from(GAME3X3));
        let first_move_valid = super::validator::validate_first_move(&history);
        assert_eq!(first_move_valid, true);
        let (result1, score, breaks) = super::validator::validate_history(history);
        assert_eq!(result1, true);
        assert_eq!(score, 14220);
        assert_eq!(breaks, 0);
    }
    #[test]
    fn validator_works_looong_4x4_0breaks() {
        use super::lib_testgames::GAMEOBSCENE;
        let history = super::parser::parse_data(String::from(GAMEOBSCENE));
        let first_move_valid = super::validator::validate_first_move(&history);
        assert_eq!(first_move_valid, true);
        let (result1, score, breaks) = super::validator::validate_history(history);
        assert_eq!(result1, true);
        assert_eq!(score, 200028);
        assert_eq!(breaks, 0);
    }
}
