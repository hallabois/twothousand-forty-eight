pub mod board;
pub mod parser;
pub mod validator;
mod direction;
mod recording;

pub const DEBUG_INFO: bool = false;

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
    fn parser_works(){
        let history4x4 = super::parser::parse_data(String::from(GAME4X4));
        assert_eq!(history4x4.width, 4);
        assert_eq!(history4x4.height, 4);
        assert_eq!(history4x4.history.len(), 576);
    }
    #[test]
    fn validator_works() {
        let history = super::parser::parse_data(String::from(GAME4X4));
        let first_move_valid = super::validator::validate_first_move(&history);
        assert_eq!(first_move_valid, true);
        let (result1, score, breaks) = super::validator::validate_history(history);
        assert_eq!(result1, true);
        assert_eq!(score, 6052);
        assert_eq!(breaks, 2);
    }
    const GAME4X4: &str = "4x4S0.0.0.0.2.0.0.0.0.2.0.0.0.0.0.0+3,2.2;0:2.2.0.0.0.0.0.0.0.0.0.2.0.0.0.0+1,2.2;3:4.0.0.0.0.0.0.0.2.2.0.0.0.0.0.0+2,2.2;0:4.2.0.0.2.0.0.0.0.0.2.0.0.0.0.0+3,0.2;3:4.2.0.2.2.0.0.0.2.0.0.0.0.0.0.0+0,2.2;0:4.2.0.2.4.0.0.0.2.0.0.0.0.0.0.0+1,1.2;3:4.4.0.0.4.2.0.0.2.0.0.0.0.0.0.0+3,1.2;0:8.4.0.0.2.2.0.2.0.0.0.0.0.0.0.0+3,0.2;3:8.4.0.2.4.2.0.0.0.0.0.0.0.0.0.0+0,3.2;3:8.4.2.0.4.2.0.0.0.0.0.0.2.0.0.0+2,2.2;0:8.4.2.0.4.2.0.0.2.0.2.0.0.0.0.0+1,2.2;3:8.4.2.0.4.2.0.0.4.2.0.0.0.0.0.0+3,0.2;0:8.4.2.2.8.4.0.0.0.0.0.0.0.0.0.0+0,2.2;3:8.4.4.0.8.4.0.0.2.0.0.0.0.0.0.0+1,2.2;0:16.8.4.0.2.0.0.0.0.2.0.0.0.0.0.0+3,0.2;3:16.8.4.2.2.0.0.0.2.0.0.0.0.0.0.0+3,1.4;0:16.8.4.2.4.0.0.4.0.0.0.0.0.0.0.0+2,1.2;3:16.8.4.2.8.0.2.0.0.0.0.0.0.0.0.0+1,3.4;3:16.8.4.2.8.2.0.0.0.0.0.0.0.4.0.0+0,2.2;0:16.8.4.2.8.2.0.0.2.4.0.0.0.0.0.0+0,1.2;1:16.8.4.2.2.0.8.2.0.0.2.4.0.0.0.0+1,3.2;0:16.8.4.4.2.0.8.4.0.0.2.0.0.2.0.0+2,2.2;3:16.8.8.0.2.8.4.0.2.0.2.0.2.0.0.0+3,3.4;0:16.16.8.0.4.0.4.0.2.0.2.0.0.0.0.4+2,0.2;3:32.8.2.0.8.0.0.0.4.0.0.0.4.0.0.0+0,3.4;0:32.8.2.0.8.0.0.0.8.0.0.0.4.0.0.0+3,0.2;0:32.8.2.2.16.0.0.0.4.0.0.0.0.0.0.0+0,3.2;3:32.8.4.0.16.0.0.0.4.0.0.0.2.0.0.0+0,0.2;1:2.32.8.4.0.0.0.16.0.0.0.4.0.0.0.2+3,3.2;3:2.32.8.4.16.0.0.0.4.0.0.0.2.0.0.2+1,1.2;0:2.32.8.4.16.2.0.2.4.0.0.0.2.0.0.0+3,2.2;3:2.32.8.4.16.4.0.0.4.0.0.2.2.0.0.0+2,2.2;0:2.32.8.4.16.4.0.2.4.0.2.0.2.0.0.0+2,3.2;3:2.32.8.4.16.4.2.0.4.2.0.0.2.0.2.0+2,2.2;0:2.32.8.4.16.4.4.0.4.2.2.0.2.0.0.0+3,2.2;3:2.32.8.4.16.8.0.0.4.4.0.2.2.0.0.0+1,3.2;0:2.32.8.4.16.8.0.2.4.4.0.0.2.2.0.0+3,3.2;3:2.32.8.4.16.8.2.0.8.0.0.0.4.0.0.2+2,3.2;0:2.32.8.4.16.8.2.2.8.0.0.0.4.0.2.0+0,1.2;1:2.32.8.4.2.16.8.4.0.0.0.8.0.0.4.2+2,3.2;0:4.32.16.8.0.16.4.8.0.0.0.2.0.0.2.0+3,2.4;3:4.32.16.8.16.4.8.0.2.0.0.4.2.0.0.0+3,2.2;0:4.32.16.8.16.4.8.4.4.0.0.2.0.0.0.0+3,3.2;1:4.32.16.8.16.4.8.4.0.0.4.2.0.0.0.2+3,2.4;3:4.32.16.8.16.4.8.4.4.2.0.4.2.0.0.0+2,3.2;1:4.32.16.8.16.4.8.4.0.4.2.4.0.0.2.2+0,2.2;0:4.32.16.8.16.8.8.8.2.0.4.2.0.0.0.0+3,2.2;0:4.32.16.16.16.8.8.2.2.0.4.2.0.0.0.0+2,3.2;1:0.4.32.32.0.16.16.2.0.2.4.2.0.0.2.0+0,2.2;1:0.0.4.64.0.0.32.2.2.2.4.2.0.0.0.2+2,1.2;3:4.64.0.0.32.2.2.0.4.4.2.0.2.0.0.0+2,2.2;0:4.64.4.0.32.2.0.0.4.4.2.0.2.0.0.0+2,3.2;3:4.64.4.0.32.2.0.0.8.2.0.0.2.0.2.0+3,1.2;0:4.64.4.0.32.4.2.2.8.0.0.0.2.0.0.0+1,2.2;3:4.64.4.0.32.4.4.0.8.2.0.0.2.0.0.0+2,3.2;0:4.64.8.0.32.4.0.0.8.2.0.0.2.0.2.0+3,0.2;3:4.64.8.2.32.4.0.0.8.2.0.0.4.0.0.0+1,1.2;1:4.64.8.2.0.2.32.4.0.0.8.2.0.0.0.4+1,3.2;3:4.64.8.2.2.32.4.0.8.2.0.0.4.2.0.0+0,3.2;1:4.64.8.2.0.2.32.4.0.0.8.2.2.0.4.2+1,2.2;0:4.64.8.2.2.2.32.4.0.2.8.4.0.0.4.0+3,1.2;3:4.64.8.2.4.32.4.2.2.8.4.0.4.0.0.0+3,3.4;0:8.64.8.4.2.32.8.0.4.8.0.0.0.0.0.4+2,2.2;3:8.64.8.4.2.32.8.0.4.8.2.0.4.0.0.0+3,1.2;0:8.64.16.4.2.32.2.2.8.8.0.0.0.0.0.0+1,2.2;3:8.64.16.4.2.32.4.0.16.2.0.0.0.0.0.0+0,2.2;1:8.64.16.4.0.2.32.4.2.0.16.2.0.0.0.0+3,2.2;0:8.64.16.8.2.2.32.2.0.0.16.2.0.0.0.0+1,3.2;3:8.64.16.8.4.32.2.0.16.2.0.0.0.2.0.0+2,3.2;0:8.64.16.8.4.32.2.0.16.4.0.0.0.0.2.0+0,3.2;1:8.64.16.8.0.4.32.2.0.0.16.4.2.0.0.2+1,3.2;0:8.64.16.8.2.4.32.2.0.0.16.4.0.2.0.2+2,3.2;3:8.64.16.8.2.4.32.2.16.4.0.0.4.0.2.0+1,2.2;0:8.64.16.8.2.8.32.2.16.2.2.0.4.0.0.0+3,2.2;3:8.64.16.8.2.8.32.2.16.4.0.2.4.0.0.0+3,3.2;0:8.64.16.8.2.8.32.4.16.4.0.0.4.0.0.2+0,3.2;1:8.64.16.8.2.8.32.4.0.0.16.4.2.0.4.2+1,3.2;0:8.64.16.8.4.8.32.8.0.0.16.2.0.2.4.0+3,2.2;3:8.64.16.8.4.8.32.8.16.2.0.2.2.4.0.0+3,2.2;0:8.64.16.16.4.8.32.2.16.2.0.2.2.4.0.0+2,2.2;3:8.64.32.0.4.8.32.2.16.4.2.0.2.4.0.0+3,1.2;0:8.64.64.2.4.8.2.2.16.8.0.0.2.0.0.0+2,2.4;3:8.128.2.0.4.8.4.0.16.8.4.0.2.0.0.0+2,2.2;0:8.128.2.0.4.16.8.0.16.0.2.0.2.0.0.0+3,0.2;3:8.128.2.2.4.16.8.0.16.2.0.0.2.0.0.0+3,3.2;3:8.128.4.0.4.16.8.0.16.2.0.0.2.0.0.2+1,3.4;0:8.128.4.2.4.16.8.0.16.2.0.0.2.4.0.0+0,1.2;1:8.128.4.2.2.4.16.8.0.0.16.2.0.0.2.4+2,3.2;0:8.128.4.2.2.4.32.8.0.0.2.2.0.0.2.4+2,3.2;3:8.128.4.2.2.4.32.8.4.0.0.0.2.4.2.0+2,3.2;0:8.128.4.2.2.8.32.8.4.0.2.0.2.0.2.0+2,2.2;3:8.128.4.2.2.8.32.8.4.2.2.0.4.0.0.0+0,3.2;0:8.128.4.2.2.8.32.8.8.2.2.0.2.0.0.0+3,3.2;3:8.128.4.2.2.8.32.8.8.4.0.0.2.0.0.2+1,3.2;0:8.128.4.2.2.8.32.8.8.4.0.2.2.2.0.0+3,3.2;3:8.128.4.2.2.8.32.8.8.4.2.0.4.0.0.2+3,3.2;0:8.128.4.2.2.8.32.8.8.4.2.2.4.0.0.2+3,3.2;3:8.128.4.2.2.8.32.8.8.4.4.0.4.2.0.2+2,3.2;0:8.128.4.2.2.8.32.8.8.4.4.2.4.2.2.0+2,3.2;3:8.128.4.2.2.8.32.8.8.8.2.0.4.4.2.0+2,3.2;0:8.128.4.2.2.16.32.8.8.4.4.0.4.0.2.0+0,2.2;1:8.128.4.2.2.16.32.8.2.0.8.8.0.0.4.2+1,3.2;0:8.128.4.2.4.16.32.16.0.0.8.2.0.2.4.0+2,3.2;3:8.128.4.2.4.16.32.16.8.2.0.0.2.4.2.0+3,3.2;0:8.128.4.2.4.16.32.16.8.2.2.0.2.4.0.2+3,2.2;3:8.128.4.2.4.16.32.16.8.4.0.2.2.4.2.0+1,3.2;0:8.128.4.2.4.16.32.16.8.8.2.2.2.2.0.0+3,3.2;3:8.128.4.2.4.16.32.16.16.4.0.0.4.0.0.2+2,3.2;0:8.128.4.2.4.16.32.16.16.4.0.2.4.0.2.0+2,3.2;3:8.128.4.2.4.16.32.16.16.4.2.0.4.2.2.0+3,3.2;0:8.128.4.2.4.16.32.16.16.4.4.0.4.2.0.2+3,2.2;3:8.128.4.2.4.16.32.16.16.8.0.2.4.4.0.0+3,3.4;3:8.128.4.2.4.16.32.16.16.8.2.0.8.0.0.4+2,3.2;0:8.128.4.2.4.16.32.16.16.8.2.4.8.0.2.0+2,3.2;3:8.128.4.2.4.16.32.16.16.8.2.4.8.2.2.0+3,3.2;0:8.128.4.2.4.16.32.16.16.8.4.4.8.2.0.2+0,3.2;1:8.128.4.2.4.16.32.16.0.16.8.8.2.0.8.4+1,3.2;0:8.128.4.2.4.32.32.16.2.0.16.8.0.2.0.4+1,3.2;1:8.128.4.2.0.4.64.16.0.2.16.8.0.2.2.4+0,1.2;0:8.128.4.2.2.4.64.16.0.4.16.8.0.0.2.4+1,3.2;0:8.128.4.2.2.8.64.16.0.0.16.8.0.2.2.4+3,2.2;3:8.128.4.2.2.8.64.16.16.8.0.2.4.4.0.0+3,3.2;0:8.128.4.2.2.16.64.16.16.4.0.2.4.0.0.2+0,3.2;1:8.128.4.2.2.16.64.16.0.16.4.2.2.0.4.2+2,3.2;0:8.128.4.2.4.32.64.16.0.0.8.4.0.0.2.0+0,3.2;1:8.128.4.2.4.32.64.16.0.0.8.4.2.0.0.2+1,2.2;0:8.128.4.2.4.32.64.16.2.2.8.4.0.0.0.2+3,2.2;3:8.128.4.2.4.32.64.16.4.8.4.2.2.0.0.0+2,3.2;0:8.128.4.2.8.32.64.16.2.8.4.2.0.0.2.0+0,2.2;0:16.128.4.2.2.32.64.16.2.8.4.2.0.0.2.0+0,2.2;0:16.128.4.2.4.32.64.16.2.8.4.2.0.0.2.0+1,3.2;3:16.128.4.2.4.32.64.16.2.8.4.2.2.2.0.0+0,3.2;0:16.128.4.2.4.32.64.16.4.8.4.2.2.2.0.0+3,3.2;0:16.128.4.2.8.32.64.16.2.8.4.2.0.2.0.2+2,3.2;3:16.128.4.2.8.32.64.16.2.8.4.2.4.0.2.0+0,3.2;1:16.128.4.2.8.32.64.16.2.8.4.2.2.0.4.2+1,3.2;0:16.128.4.2.8.32.64.16.4.8.8.4.0.2.0.0+3,3.2;3:16.128.4.2.8.32.64.16.4.16.4.0.2.0.0.2+1,3.4;3:16.128.4.2.8.32.64.16.4.16.4.0.4.4.0.0+0,3.4;0:16.128.4.2.8.32.64.16.8.16.4.0.4.4.0.0+0,3.4;0:16.128.4.2.16.32.64.16.4.16.4.0.4.4.0.0+0,2.2;0:32.128.4.2.8.32.64.16.2.16.4.0.0.4.0.0+2,3.2;3:32.128.4.2.8.32.64.16.2.16.4.0.4.0.2.0+0,2.2;1:32.128.4.2.8.32.64.16.2.2.16.4.0.0.4.2+3,3.2;3:32.128.4.2.8.32.64.16.4.16.4.0.4.2.0.2+3,3.2;0:32.128.4.2.8.32.64.16.8.16.4.2.0.2.0.2+2,3.2;0:32.128.4.2.16.32.64.16.0.16.4.4.0.2.2.0+3,2.2;3:32.128.4.2.16.32.64.16.16.8.0.2.4.0.0.0+2,3.2;0:32.128.4.2.32.32.64.16.4.8.0.2.0.0.2.0+3,3.2;0:64.128.4.2.4.32.64.16.0.8.2.2.0.0.0.2+2,2.4;3:64.128.4.2.4.32.64.16.8.4.4.0.2.0.0.0+2,3.2;1:64.128.4.2.4.32.64.16.0.0.8.8.0.0.2.2+0,3.4;1:64.128.4.2.4.32.64.16.0.0.0.16.4.0.0.4+2,3.2;0:64.128.4.2.8.32.64.32.0.0.0.4.0.0.2.0+1,3.2;0:64.128.4.2.8.32.64.32.0.0.2.4.0.2.0.0+2,3.2;0:64.128.4.2.8.32.64.32.0.2.2.4.0.0.2.0+2,3.2;3:64.128.4.2.8.32.64.32.4.4.0.0.2.0.2.0+1,3.2;3:64.128.4.2.8.32.64.32.8.0.0.0.4.2.0.0+2,2.2;0:64.128.4.2.16.32.64.32.4.2.2.0.0.0.0.0+2,3.2;3:64.128.4.2.16.32.64.32.4.4.0.0.0.0.2.0+3,3.2;3:64.128.4.2.16.32.64.32.8.0.0.0.2.0.0.2+2,3.2;0:64.128.4.2.16.32.64.32.8.0.0.2.2.0.2.0+1,2.2;0:64.128.4.2.16.32.64.32.8.2.2.2.2.0.0.0+3,3.2;3:64.128.4.2.16.32.64.32.8.4.2.0.2.0.0.2+2,3.2;0:64.128.4.2.16.32.64.32.8.4.2.2.2.0.2.0+1,3.2;3:64.128.4.2.16.32.64.32.8.4.4.0.4.2.0.0+3,3.2;3:64.128.4.2.16.32.64.32.8.8.0.0.4.2.0.2+2,2.4;3:64.128.4.2.16.32.64.32.16.0.4.0.4.4.0.0+2,3.2;0:64.128.4.2.32.32.64.32.4.4.4.0.0.0.2.0+3,2.2;3:64.128.4.2.64.64.32.0.8.4.0.2.2.0.0.0+0,3.2;0:128.128.4.4.8.64.32.0.2.4.0.0.2.0.0.0+2,2.2;3:256.8.0.0.8.64.32.0.2.4.2.0.2.0.0.0+3,1.2;0:256.8.32.0.8.64.2.2.4.4.0.0.0.0.0.0+2,3.2;3:256.8.32.0.8.64.4.0.8.0.0.0.0.0.2.0+3,0.2;0:256.8.32.2.16.64.4.0.0.0.2.0.0.0.0.0+2,3.2;3:256.8.32.2.16.64.4.0.2.0.0.0.0.0.2.0+3,2.2;0:256.8.32.2.16.64.4.0.2.0.2.2.0.0.0.0+3,2.2;3:256.8.32.2.16.64.4.0.4.2.0.2.0.0.0.0+3,2.2;0:256.8.32.4.16.64.4.0.4.2.0.2.0.0.0.0+0,3.2;3:256.8.32.4.16.64.4.0.4.4.0.0.2.0.0.0+3,1.2;3:256.8.32.4.16.64.4.2.8.0.0.0.2.0.0.0+1,2.2;1:256.8.32.4.16.64.4.2.0.2.0.8.0.0.0.2+1,3.2;3:256.8.32.4.16.64.4.2.2.8.0.0.2.2.0.0+3,3.2;0:256.8.32.4.16.64.4.2.4.8.0.0.0.2.0.2+3,3.2;3:256.8.32.4.16.64.4.2.4.8.0.0.4.0.0.2+0,3.2;0:256.8.32.4.16.64.4.4.8.8.0.0.2.0.0.0+1,2.2;3:256.8.32.4.16.64.8.0.16.2.0.0.2.0.0.0+3,2.2;0:256.8.32.4.32.64.8.0.2.2.0.2.0.0.0.0+1,3.2;3:256.8.32.4.32.64.8.0.4.2.0.0.0.2.0.0+2,2.4;0:256.8.32.4.32.64.8.0.4.4.4.0.0.0.0.0+2,2.4;3:256.8.32.4.32.64.8.0.8.4.4.0.0.0.0.0+3,3.2;3:256.8.32.4.32.64.8.0.8.8.0.0.0.0.0.2+2,3.2;0:256.8.32.4.32.64.8.2.8.8.0.0.0.0.2.0+2,3.2;3:256.8.32.4.32.64.8.2.16.0.0.0.2.0.2.0+3,3.2;0:256.8.32.4.32.64.8.2.16.0.2.0.2.0.0.2+2,3.4;3:256.8.32.4.32.64.8.2.16.2.0.0.4.0.4.0+3,2.2;0:256.8.32.4.32.64.8.2.16.2.4.2.4.0.0.0+1,3.2;0:256.8.32.4.32.64.8.4.16.2.4.0.4.2.0.0+3,3.2;0:256.8.32.8.32.64.8.0.16.4.4.0.4.0.0.2+3,2.4;3:256.8.32.8.32.64.8.0.16.8.0.4.4.2.0.0+2,3.2;0:256.8.32.8.32.64.8.4.16.8.0.0.4.2.2.0+3,3.2;3:256.8.32.8.32.64.8.4.16.8.0.0.4.4.0.2+2,3.2;0:256.8.32.8.32.64.8.4.16.8.0.2.4.4.2.0+3,2.4;3:256.8.32.8.32.64.8.4.16.8.2.4.8.2.0.0+3,2.2;0:256.8.32.8.32.64.8.8.16.8.2.2.8.2.0.0+3,3.2;3:256.8.32.8.32.64.16.0.16.8.4.0.8.2.0.2+2,3.2;0:256.8.32.8.32.64.16.2.16.8.4.0.8.2.2.0+3,3.2;3:256.8.32.8.32.64.16.2.16.8.4.0.8.4.0.2+3,2.2;0:256.8.32.8.32.64.16.4.16.8.4.2.8.4.0.0+0,3.4;1:256.8.32.8.32.64.16.4.16.8.4.2.4.0.8.4+3,3.2;3:256.8.32.8.32.64.16.4.16.8.4.2.4.8.4.2+3,3.2;0:256.8.32.8.32.64.16.4.16.16.8.4.4.0.0.2+3,3.2;3:256.8.32.8.32.64.16.4.32.8.4.0.4.2.0.2+3,3.4;0:256.8.32.8.64.64.16.4.4.8.4.2.0.2.0.4+3,3.2;3:256.8.32.8.128.16.4.0.4.8.4.2.2.4.0.2+2,2.2;0:256.8.32.8.128.16.8.4.4.8.2.0.2.4.0.0+0,3.2;1:256.8.32.8.128.16.8.4.0.4.8.2.2.0.2.4+0,3.2;0:256.8.32.8.128.16.16.4.2.4.2.2.2.0.0.4+2,3.2;3:256.8.32.8.128.32.4.0.2.4.4.0.2.4.2.0+1,3.2;0:256.8.32.8.128.32.8.0.4.8.2.0.0.2.0.0+0,2.2;1:256.8.32.8.0.128.32.8.2.4.8.2.0.0.0.2+0,2.2;0:256.8.64.16.2.128.8.4.2.4.0.0.0.0.0.0+1,3.2;0:256.8.64.16.4.128.8.4.0.4.0.0.0.2.0.0+1,2.2;1:256.8.64.16.4.128.8.4.0.2.0.4.0.0.0.2+3,3.2;0:256.8.64.16.4.128.8.8.0.2.0.2.0.0.0.2+1,2.2;3:256.8.64.16.4.128.16.0.4.2.0.0.2.0.0.0+1,2.2;1:256.8.64.16.0.4.128.16.0.2.4.2.0.0.0.2+1,3.2;0:256.8.64.32.0.4.128.4.0.2.4.0.0.2.0.0+3,2.2;3:256.8.64.32.4.128.4.0.2.4.0.2.2.0.0.0+2,2.2;0:256.8.64.32.4.128.4.2.4.4.2.0.0.0.0.0+0,3.2;3:256.8.64.32.4.128.4.2.8.2.0.0.2.0.0.0+0,3.2;1:256.8.64.32.4.128.4.2.0.0.8.2.2.0.0.2+0,3.2;0:256.8.64.32.4.128.4.4.2.0.8.2.2.0.0.0+3,2.2;3:256.8.64.32.4.128.8.0.2.8.2.2.2.0.0.0+3,3.2;3:256.8.64.32.4.128.8.0.2.8.4.0.2.0.0.2+3,1.2;3:256.8.64.32.4.128.8.2.2.8.4.0.4.0.0.0+2,3.2;1:256.8.64.32.4.128.8.2.0.2.8.4.0.0.2.4+3,3.2;0:256.8.64.32.4.128.16.2.0.2.2.8.0.0.0.2+2,3.2;3:256.8.64.32.4.128.16.2.4.8.0.0.2.0.2.0+2,3.2;0:256.8.64.32.8.128.16.2.2.8.2.0.0.0.2.0+1,3.2;3:256.8.64.32.8.128.16.2.2.8.2.0.2.2.0.0+3,2.2;0:256.8.64.32.8.128.16.2.4.8.2.2.0.2.0.0+1,3.2;3:256.8.64.32.8.128.16.2.4.8.4.0.2.2.0.0+3,3.2;3:256.8.64.32.8.128.16.2.4.8.4.0.4.0.0.2+3,2.2;0:256.8.64.32.8.128.16.4.8.8.4.2.0.0.0.0+3,3.2;0:256.8.64.32.16.128.16.4.0.8.4.2.0.0.0.2+3,3.2;3:256.8.64.32.16.128.16.4.8.4.2.0.2.0.0.2+3,3.2;0:256.8.64.32.16.128.16.4.8.4.2.2.2.0.0.2+1,3.2;3:256.8.64.32.16.128.16.4.8.4.4.0.4.2.0.0+3,2.2;3:256.8.64.32.16.128.16.4.8.8.0.2.4.2.0.0+3,3.2;3:256.8.64.32.16.128.16.4.16.2.0.0.4.2.0.2+3,3.2;0:256.8.64.32.32.128.16.4.4.4.0.2.0.0.0.2+3,2.4;3:256.8.64.32.32.128.16.4.8.2.0.4.2.0.0.0+3,3.2;0:256.8.64.32.32.128.16.8.8.2.0.0.2.0.0.2+2,2.2;3:256.8.64.32.32.128.16.8.8.2.2.0.4.0.0.0+2,2.2;3:256.8.64.32.32.128.16.8.8.4.2.0.4.0.0.0+2,3.2;1:256.8.64.32.32.128.16.8.0.8.4.2.0.0.2.4+2,3.2;3:256.8.64.32.32.128.16.8.8.4.2.0.2.4.2.0+3,3.4;0:256.8.64.32.32.128.16.8.8.8.4.0.2.0.0.4+3,3.4;3:256.8.64.32.32.128.16.8.16.4.0.0.2.4.0.4+0,2.4;1:256.8.64.32.32.128.16.8.4.0.16.4.0.0.2.8+2,3.2;0:256.8.64.32.32.128.32.8.4.0.2.4.0.0.2.8+2,3.2;0:256.8.64.32.32.128.32.8.4.0.4.4.0.0.2.8+3,2.2;3:256.8.64.32.32.128.32.8.8.4.0.2.2.8.0.0+2,3.2;3:256.8.64.32.32.128.32.8.8.4.2.0.2.8.2.0+3,2.2;0:256.8.64.32.32.128.32.8.8.4.4.2.2.8.0.0+2,3.2;3:256.8.64.32.32.128.32.8.8.8.2.0.2.8.2.0+2,3.4;0:256.8.64.32.32.128.32.8.8.16.4.0.2.0.4.0+3,2.2;3:256.8.64.32.32.128.32.8.8.16.4.2.2.4.0.0+0,3.2;1:256.8.64.32.32.128.32.8.8.16.4.2.2.0.2.4+3,3.2;3:256.8.64.32.32.128.32.8.8.16.4.2.4.4.0.2+3,3.2;0:256.8.64.32.32.128.32.8.8.16.4.4.4.4.0.2+3,3.2;3:256.8.64.32.32.128.32.8.8.16.8.0.8.2.0.2+0,3.2;0:256.8.64.32.32.128.32.8.16.16.8.2.2.2.0.0+1,3.2;3:256.8.64.32.32.128.32.8.32.8.2.0.4.2.0.0+2,3.2;0:256.8.64.32.64.128.32.8.4.8.2.0.0.2.2.0+3,2.2;3:256.8.64.32.64.128.32.8.4.8.2.2.4.0.0.0+2,3.2;0:256.8.64.32.64.128.32.8.8.8.2.2.0.0.2.0+1,3.2;3:256.8.64.32.64.128.32.8.16.4.0.0.2.2.0.0+3,3.2;3:256.8.64.32.64.128.32.8.16.4.0.0.4.0.0.2+1,3.2;1:256.8.64.32.64.128.32.8.0.0.16.4.0.2.4.2+3,3.2;3:256.8.64.32.64.128.32.8.16.4.0.0.2.4.2.2+2,3.4;0:256.8.64.32.64.128.32.8.16.8.2.2.2.0.4.0+2,3.2;3:256.8.64.32.64.128.32.8.16.8.4.0.2.4.2.0+0,2.2;1:256.8.64.32.64.128.32.8.2.16.8.4.0.2.4.2+3,3.4;3:256.8.64.32.64.128.32.8.2.16.8.4.2.4.2.4+3,3.2;0:256.8.64.32.64.128.32.8.4.16.8.8.0.4.2.2+3,3.2;3:256.8.64.32.64.128.32.8.4.16.16.0.4.4.0.2+2,2.2;3:256.8.64.32.64.128.32.8.4.32.2.0.8.2.0.0+0,2.2;1:256.8.64.32.64.128.32.8.2.4.32.2.0.0.8.2+3,3.2;0:256.8.64.32.64.128.64.8.2.4.8.4.0.0.0.2+1,3.2;0:256.8.128.32.64.128.8.8.2.4.0.4.0.2.0.2+0,3.2;1:256.8.128.32.0.64.128.16.0.0.2.8.2.0.0.4+0,2.4;0:256.8.256.32.2.64.2.16.4.0.0.8.0.0.0.4+2,3.2;3:256.8.256.32.2.64.2.16.4.8.0.0.4.0.2.0+3,3.2;0:256.8.256.32.2.64.4.16.8.8.0.0.0.0.0.2+2,3.2;1:256.8.256.32.2.64.4.16.0.0.0.16.0.0.2.2+1,2.2;0:256.8.256.32.2.64.4.32.0.2.2.2.0.0.0.0+2,3.2;0:256.8.256.64.2.64.4.2.0.2.2.0.0.0.2.0+1,2.2;1:256.8.256.64.2.64.4.2.0.2.0.4.0.0.0.2+1,3.2;3:256.8.256.64.2.64.4.2.2.4.0.0.2.2.0.0+2,2.2;0:256.8.256.64.4.64.4.2.2.4.2.0.0.2.0.0+1,3.2;1:256.8.256.64.4.64.4.2.0.2.4.2.0.2.0.2+0,2.2;0:256.8.256.64.4.64.8.4.2.4.0.2.0.0.0.0+0,2.4;1:256.8.256.64.4.64.8.4.4.2.4.2.0.0.0.0+0,2.2;0:256.8.256.64.8.64.8.4.2.2.4.2.0.0.0.0+1,3.4;3:256.8.256.64.8.64.8.4.4.4.2.0.0.4.0.0+2,2.2;3:256.8.256.64.8.64.8.4.8.2.2.0.4.0.0.0+1,3.2;0:256.8.256.64.16.64.8.4.4.2.2.0.0.2.0.0+0,3.2;1:256.8.256.64.16.64.8.4.0.0.4.4.2.0.0.2+2,3.2;0:256.8.256.64.16.64.8.8.2.0.4.2.0.0.2.0+3,1.2;3:256.8.256.64.16.64.16.2.2.4.2.0.2.0.0.0+2,3.2;1:256.8.256.64.16.64.16.2.0.2.4.2.0.0.2.2+1,3.2;0:256.8.256.64.16.64.16.4.0.2.4.2.0.2.2.0+0,3.2;1:256.8.256.64.16.64.16.4.0.2.4.2.2.0.0.4+1,3.2;0:256.8.256.64.16.64.16.4.2.2.4.2.0.2.0.4+3,2.2;3:256.8.256.64.16.64.16.4.4.4.2.2.2.4.0.0+1,3.2;1:256.8.256.64.16.64.16.4.0.0.8.4.0.2.2.4+0,3.2;0:256.8.256.64.16.64.16.8.0.2.8.4.2.0.2.0+3,2.2;3:256.8.256.64.16.64.16.8.2.8.4.2.4.0.0.0+0,3.4;1:256.8.256.64.16.64.16.8.2.8.4.2.4.0.0.4+1,3.2;3:256.8.256.64.16.64.16.8.2.8.4.2.8.2.0.0+0,3.2;1:256.8.256.64.16.64.16.8.2.8.4.2.2.0.8.2+3,3.2;3:256.8.256.64.16.64.16.8.2.8.4.2.2.8.2.2+0,3.2;0:256.8.256.64.16.64.16.8.4.16.4.4.2.0.2.0+2,3.4;1:256.8.256.64.16.64.16.8.0.4.16.8.0.0.4.4+3,3.4;0:256.8.256.64.16.64.32.16.0.4.4.4.0.0.0.4+0,3.2;1:256.8.256.64.16.64.32.16.0.0.4.8.2.0.0.4+0,2.2;1:256.8.256.64.16.64.32.16.2.0.4.8.0.0.2.4+1,3.2;1:256.8.256.64.16.64.32.16.0.2.4.8.0.2.2.4+0,2.2;1:256.8.256.64.16.64.32.16.2.2.4.8.0.0.4.4+2,3.2;0:256.8.256.64.16.64.32.16.2.2.8.8.0.0.2.4+1,2.2;1:256.8.256.64.16.64.32.16.0.2.4.16.0.0.2.4+0,2.2;0:256.8.256.64.16.64.32.32.2.2.4.4.0.0.2.0+0,3.2;1:256.8.256.64.0.16.64.64.0.0.4.8.2.0.0.2+3,3.4;0:256.8.256.128.2.16.64.8.0.0.4.2.0.0.0.4+3,2.2;3:256.8.256.128.2.16.64.8.4.2.0.2.4.0.0.0+3,3.2;0:256.8.256.128.2.16.64.8.8.2.0.2.0.0.0.2+1,3.2;1:256.8.256.128.2.16.64.8.0.0.8.4.0.2.0.2+1,3.2;0:256.8.256.128.2.16.64.8.0.2.8.4.0.2.0.2+3,2.2;3:256.8.256.128.2.16.64.8.2.8.4.2.4.0.0.0+1,3.2;0:256.8.256.128.4.16.64.8.4.8.4.2.0.2.0.0+0,3.2;0:256.8.256.128.8.16.64.8.0.8.4.2.2.2.0.0+3,3.2;3:256.8.256.128.8.16.64.8.8.4.2.0.4.0.0.2+0,3.2;0:256.8.256.128.16.16.64.8.4.4.2.2.2.0.0.0+2,2.2;3:256.0.256.128.32.64.0.0.0.0.0.0.0.0.0.0+0,3.2;3:512.128.0.0.32.64.0.0.0.0.0.0.2.0.0.0+3,3.4;0:512.128.0.0.32.64.0.0.2.0.0.0.0.0.0.4+3,0.2;3:512.128.0.2.32.64.0.0.2.0.0.0.4.0.0.0+1,3.2;3:512.128.2.0.32.64.0.0.2.0.0.0.4.2.0.0+3,3.4;0:512.128.2.0.32.64.0.0.2.2.0.0.4.0.0.4+2,2.2;3:512.128.2.0.32.64.0.0.4.0.2.0.8.0.0.0+2,2.2;0:512.128.4.0.32.64.0.0.4.0.2.0.8.0.0.0+3,0.4;3:512.128.4.4.32.64.0.0.4.2.0.0.8.0.0.0+3,3.2;3:512.128.8.0.32.64.0.0.4.2.0.0.8.0.0.2+3,3.2;0:512.128.8.2.32.64.0.0.4.2.0.0.8.0.0.2+3,3.2;3:512.128.8.2.32.64.0.0.4.2.0.0.8.2.0.2+1,3.2;0:512.128.8.4.32.64.0.0.4.4.0.0.8.2.0.0+2,3.2;3:512.128.8.4.32.64.0.0.8.0.0.0.8.2.2.0+3,2.2;0:512.128.8.4.32.64.2.0.16.2.0.2.0.0.0.0+3,3.2;3:512.128.8.4.32.64.2.0.16.4.0.0.0.0.0.2+3,3.2;0:512.128.8.4.32.64.2.2.16.4.0.0.0.0.0.2+3,1.2;3:512.128.8.4.32.64.4.2.16.4.0.0.2.0.0.0+0,2.4;1:512.128.8.4.32.64.4.2.4.0.16.4.0.0.0.2+2,3.2;3:512.128.8.4.32.64.4.2.4.16.4.0.2.0.2.0+3,3.2;0:512.128.8.4.32.64.8.2.4.16.2.0.2.0.0.2+3,2.2;0:512.128.16.4.32.64.2.4.4.16.0.2.2.0.0.0+3,3.2;3:512.128.16.4.32.64.2.4.4.16.2.0.2.0.0.2+2,2.2;0:512.128.16.8.32.64.4.2.4.16.2.0.2.0.0.0+2,3.2;1:512.128.16.8.32.64.4.2.0.4.16.2.0.0.2.2+0,2.2;0:512.128.16.8.32.64.4.4.2.4.16.2.0.0.2.0+2,3.2;3:512.128.16.8.32.64.8.0.2.4.16.2.2.0.2.0+1,3.2;1:512.128.16.8.0.32.64.8.2.4.16.2.0.2.0.4+0,2.2;0:512.128.16.16.2.32.64.2.2.4.16.4.0.2.0.0+3,3.4;3:512.128.32.0.2.32.64.2.2.4.16.4.2.0.0.4+0,3.2;0:512.128.32.2.4.32.64.8.2.4.16.0.2.0.0.0+3,2.2;0:512.128.32.2.4.32.64.8.4.4.16.2.0.0.0.0+0,3.2;0:512.128.32.2.8.32.64.8.0.4.16.2.2.0.0.0+2,3.2;0:512.128.32.2.8.32.64.8.2.4.16.2.0.0.2.0+1,3.2;3:512.128.32.2.8.32.64.8.2.4.16.2.2.2.0.0+2,3.2;0:512.128.32.2.8.32.64.8.4.4.16.2.0.2.2.0+3,3.2;3:512.128.32.2.8.32.64.8.8.16.2.0.4.0.0.2+1,3.2;0:512.128.32.2.16.32.64.8.4.16.2.2.0.2.0.0+3,2.2;3:512.128.32.2.16.32.64.8.4.16.4.2.2.0.0.0+0,3.2;1:512.128.32.2.16.32.64.8.4.16.4.2.2.0.0.2+2,3.2;0:512.128.32.2.16.32.64.8.4.16.4.4.2.0.2.0+3,3.2;3:512.128.32.2.16.32.64.8.4.16.8.0.4.0.0.2+3,3.2;0:512.128.32.2.16.32.64.8.8.16.8.2.0.0.0.2+2,3.4;0:512.128.32.2.16.32.64.8.8.16.8.4.0.0.4.0+2,3.2;1:512.128.32.2.16.32.64.8.8.16.8.4.0.0.2.4+3,3.2;0:512.128.32.2.16.32.64.8.8.16.8.8.0.0.2.2+0,3.2;1:512.128.32.2.16.32.64.8.0.8.16.16.2.0.0.4+0,3.2;0:512.128.32.2.16.32.64.8.2.8.16.16.2.0.0.4+1,3.2;0:512.128.32.2.16.32.64.8.4.8.16.16.0.2.0.4+3,2.4;3:512.128.32.2.16.32.64.8.4.8.32.4.2.4.0.0+0,3.2;1:512.128.32.2.16.32.64.8.4.8.32.4.2.0.2.4+3,3.2;3:512.128.32.2.16.32.64.8.4.8.32.4.4.4.0.2+2,3.2;0:512.128.32.2.16.32.64.8.8.8.32.4.0.4.2.2+3,2.2;3:512.128.32.2.16.32.64.8.16.32.4.2.4.4.0.0+1,3.2;0:512.128.32.2.32.64.64.8.4.4.4.2.0.2.0.0+3,1.2;3:512.128.32.2.32.128.8.2.8.4.2.0.2.0.0.0+1,3.2;0:512.256.32.4.32.4.8.0.8.0.2.0.2.2.0.0+2,3.2;3:512.256.32.4.32.4.8.0.8.2.0.0.4.0.2.0+1,2.2;1:512.256.32.4.0.32.4.8.0.2.8.2.0.0.4.2+0,1.2;0:512.256.32.4.2.32.4.8.0.2.8.4.0.0.4.0+3,2.4;3:512.256.32.4.2.32.4.8.2.8.4.4.4.0.0.0+1,3.2;0:512.256.32.4.4.32.8.8.4.8.0.4.0.2.0.0+1,3.2;3:512.256.32.4.4.32.16.0.4.8.4.0.2.2.0.0+2,3.2;3:512.256.32.4.4.32.16.0.4.8.4.0.4.0.2.0+3,1.2;0:512.256.32.4.8.32.16.2.4.8.4.0.0.0.2.0+2,3.2;3:512.256.32.4.8.32.16.2.4.8.4.0.2.0.2.0+1,3.2;3:512.256.32.4.8.32.16.2.4.8.4.0.4.2.0.0+2,3.2;0:512.256.32.4.8.32.16.2.8.8.4.0.0.2.2.0+3,3.2;0:512.256.32.4.16.32.16.2.0.8.4.0.0.2.2.2+3,3.2;3:512.256.32.4.16.32.16.2.8.4.0.0.4.2.0.2+3,2.2;3:512.256.32.4.16.32.16.2.8.4.0.2.4.4.0.0+0,3.2;1:512.256.32.4.16.32.16.2.0.8.4.2.2.0.0.8+2,3.2;3:512.256.32.4.16.32.16.2.8.4.2.0.2.8.2.0+3,3.2;0:512.256.32.4.16.32.16.2.8.4.4.0.2.8.0.2+3,3.2;3:512.256.32.4.16.32.16.2.8.8.0.0.2.8.2.2+3,3.2;3:512.256.32.4.16.32.16.2.16.0.0.0.2.8.4.2+3,2.2;0:512.256.32.4.32.32.16.4.2.8.4.2.0.0.0.0+3,3.2;1:512.256.32.4.0.64.16.4.2.8.4.2.0.0.0.2+0,3.4;0:512.256.32.8.2.64.16.4.0.8.4.0.4.0.0.0+0,2.2;1:512.256.32.8.2.64.16.4.2.0.8.4.0.0.0.4+1,2.2;0:512.256.32.8.4.64.16.8.0.2.8.4.0.0.0.0+3,3.2;0:512.256.32.16.4.64.16.4.0.2.8.0.0.0.0.2+0,2.2;0:512.256.32.16.4.64.16.4.2.2.8.2.0.0.0.0+3,3.2;3:512.256.32.16.4.64.16.4.4.8.2.0.0.0.0.2+1,3.2;0:512.256.32.16.8.64.16.4.0.8.2.2.0.2.0.0+0,3.4;1:512.256.32.16.8.64.16.4.0.0.8.4.4.0.0.2+1,3.2;0:512.256.32.16.8.64.16.8.4.0.8.2.0.2.0.0+1,3.2;1:512.256.32.16.8.64.16.8.0.4.8.2.0.2.0.2+0,3.2;0:512.256.32.16.8.64.16.8.0.4.8.4.2.2.0.0+1,3.2;1:512.256.32.16.8.64.16.8.0.4.8.4.0.2.0.4+0,2.2;0:512.256.32.16.8.64.16.8.2.4.8.8.0.2.0.0+0,3.2;0:512.256.32.16.8.64.16.16.2.4.8.0.2.2.0.0+2,3.2;0:512.256.32.32.8.64.16.0.4.4.8.0.0.2.2.0+3,1.2;3:512.256.64.0.8.64.16.2.8.8.0.0.4.0.0.0+3,2.2;0:512.256.64.2.16.64.16.0.4.8.0.2.0.0.0.0+3,1.2;0:512.256.64.4.16.64.16.2.4.8.0.0.0.0.0.0+2,3.2;1:512.256.64.4.16.64.16.2.0.0.4.8.0.0.2.0+1,3.2;3:512.256.64.4.16.64.16.2.4.8.0.0.2.2.0.0+2,3.2;3:512.256.64.4.16.64.16.2.4.8.0.0.4.0.2.0+2,3.2;0:512.256.64.4.16.64.16.2.8.8.2.0.0.0.2.0+0,2.2;1:512.256.64.4.16.64.16.2.2.0.16.2.0.0.0.2+1,2.2;0:512.256.64.4.16.64.32.4.2.2.0.2.0.0.0.0+3,2.2;3:512.256.64.4.16.64.32.4.4.2.0.2.0.0.0.0+0,3.2;0:512.256.64.8.16.64.32.2.4.2.0.0.2.0.0.0+1,2.2;1:512.256.64.8.16.64.32.2.0.2.4.2.0.0.0.2+0,2.2;0:512.256.64.8.16.64.32.4.2.2.4.2.0.0.0.0+1,3.2;1:512.256.64.8.16.64.32.4.0.4.4.2.0.2.0.0+1,3.2;1:512.256.64.8.16.64.32.4.0.0.8.2.0.2.0.2+3,3.2;0:512.256.64.8.16.64.32.4.0.2.8.4.0.0.0.2+1,3.2;0:512.256.64.8.16.64.32.8.0.2.8.2.0.2.0.0+1,3.2;0:512.256.64.16.16.64.32.2.0.4.8.0.0.2.0.0+2,3.2;3:512.256.64.16.16.64.32.2.4.8.0.0.2.0.2.0+2,3.2;0:512.256.64.16.16.64.32.2.4.8.2.0.2.0.2.0+2,3.2;1:512.256.64.16.16.64.32.2.0.4.8.2.0.0.2.4+0,3.2;0:512.256.64.16.16.64.32.4.0.4.8.4.2.0.2.0+1,3.2;0:512.256.64.16.16.64.32.8.2.4.8.0.0.2.2.0+2,3.2;1:512.256.64.16.16.64.32.8.0.2.4.8.0.0.2.4+0,2.2;0:512.256.64.16.16.64.32.16.2.2.4.4.0.0.2.0+1,3.2;0:512.256.64.32.16.64.32.4.2.2.4.0.0.2.2.0+1,2.2;1:512.256.64.32.16.64.32.4.0.2.4.4.0.0.0.4+3,3.4;0:512.256.64.32.16.64.32.8.0.2.4.4.0.0.0.4+3,3.2;0:512.256.64.32.16.64.32.8.0.2.4.8.0.0.0.2+1,3.2;0:512.256.64.32.16.64.32.16.0.2.4.2.0.2.0.0+2,3.2;1:512.256.64.32.16.64.32.16.0.2.4.2.0.0.2.2+1,3.2;0:512.256.64.32.16.64.32.16.0.2.4.4.0.2.2.0+0,2.2;1:512.256.64.32.16.64.32.16.2.0.2.8.0.0.0.4+1,2.2;1:512.256.64.32.16.64.32.16.0.2.4.8.0.0.0.4+2,3.2;3:512.256.64.32.16.64.32.16.2.4.8.0.4.0.2.0+0,2.2;1:512.256.64.32.16.64.32.16.2.2.4.8.0.0.4.2+1,3.2;0:512.256.64.32.16.64.32.16.2.2.8.8.0.2.0.2+0,2.2;1:512.256.64.32.16.64.32.16.2.0.4.16.0.0.0.4+2,3.2;0:512.256.64.32.16.64.32.32.2.0.4.4.0.0.2.0+1,2.2;0:512.256.64.64.16.64.32.4.2.2.4.0.0.0.2.0+2,2.2;3:512.256.128.0.16.64.32.4.4.4.2.0.2.0.0.0+0,0.4;1:4.512.256.128.16.64.32.4.0.0.8.2.0.0.0.2+2,3.2;0:4.512.256.128.16.64.32.4.0.0.8.4.0.0.2.0+0,2.4;1:4.512.256.128.16.64.32.4.4.0.8.4.0.0.0.2+1,2.2;0:0.512.256.128.16.64.32.0.0.0.0.0.0.0.0.0+2,3.2;0:16.512.256.128.0.64.32.0.0.0.0.0.0.0.2.0+2,2.2;1:16.512.256.128.0.0.64.32.0.0.2.0.0.0.0.2+0,1.2;1:16.512.256.128.2.0.64.32.0.0.0.2.0.0.0.2+1,2.2;0:16.512.256.128.2.0.64.32.0.2.0.4.0.0.0.0+2,3.2;0:16.512.256.128.2.2.64.32.0.0.0.4.0.0.2.0+2,3.2;3:16.512.256.128.4.64.32.0.4.0.0.0.2.0.2.0+3,2.2;0:16.512.256.128.8.64.32.0.2.0.2.2.0.0.0.0+3,3.2;1:16.512.256.128.0.8.64.32.0.0.2.4.0.0.0.2+2,2.2;3:16.512.256.128.8.64.32.0.2.4.2.0.2.0.0.0+3,2.2;0:16.512.256.128.8.64.32.0.4.4.2.2.0.0.0.0+2,2.2;3:16.512.256.128.8.64.32.0.8.4.2.0.0.0.0.0+0,3.2;0:16.512.256.128.16.64.32.0.0.4.2.0.2.0.0.0+1,3.2;0:32.512.256.128.2.64.32.0.0.4.2.0.0.2.0.0+0,3.4;1:32.512.256.128.0.2.64.32.0.0.4.2.4.0.0.2+1,2.2;1:32.512.256.128.0.2.64.32.0.2.4.2.0.0.4.2+0,1.2;0:32.512.256.128.2.4.64.32.0.0.8.4.0.0.0.0+3,2.4;3:32.512.256.128.2.4.64.32.8.4.0.4.0.0.0.0+0,3.2;0:32.512.256.128.2.8.64.32.8.0.0.4.2.0.0.0+1,3.4;1:32.512.256.128.2.8.64.32.0.0.8.4.0.4.0.2+0,2.4;0:32.512.256.128.2.8.64.32.4.4.8.4.0.0.0.2+3,2.2;3:32.512.256.128.2.8.64.32.8.8.4.2.2.0.0.0+2,3.2;0:32.512.256.128.2.16.64.32.8.0.4.2.2.0.2.0+0,3.2;1:32.512.256.128.2.16.64.32.0.8.4.2.2.0.0.4+2,3.2;0:32.512.256.128.4.16.64.32.0.8.4.2.0.0.2.4+3,2.2;3:32.512.256.128.4.16.64.32.8.4.2.2.2.4.0.0+1,3.2;0:32.512.256.128.4.16.64.32.8.8.2.2.2.2.0.0+2,2.2;3:32.512.256.128.4.16.64.32.16.4.2.0.4.0.0.0+2,3.2;1:32.512.256.128.4.16.64.32.0.16.4.2.0.0.2.4+1,3.2;0:32.512.256.128.4.32.64.32.0.0.4.2.0.2.2.4+0,2.2;0:32.512.256.128.4.32.64.32.2.2.4.2.0.0.2.4+2,3.2;3:32.512.256.128.4.32.64.32.4.4.2.0.2.4.2.0+3,3.2;0:32.512.256.128.8.32.64.32.2.8.4.0.0.0.0.2+3,3.2;3:32.512.256.128.8.32.64.32.2.8.4.0.2.0.0.2+1,3.2;0:32.512.256.128.8.32.64.32.4.8.4.2.0.2.0.0+2,3.2;3:32.512.256.128.8.32.64.32.4.8.4.2.2.0.2.0+2,3.2;3:32.512.256.128.8.32.64.32.4.8.4.2.4.0.2.0+0,3.2;0:32.512.256.128.8.32.64.32.8.8.4.2.2.0.2.0+1,3.2;0:32.512.256.128.16.32.64.32.2.8.4.2.0.2.2.0+3,3.2;3:32.512.256.128.16.32.64.32.2.8.4.2.4.0.0.2+1,3.2;0:32.512.256.128.16.32.64.32.2.8.4.4.4.2.0.0+0,2.2;1:32.512.256.128.16.32.64.32.2.2.8.8.0.0.4.2+1,2.2;1:32.512.256.128.16.32.64.32.0.2.4.16.0.0.4.2+3,3.2;3:32.512.256.128.16.32.64.32.2.4.16.0.4.2.0.2+0,2.2;1:32.512.256.128.16.32.64.32.2.2.4.16.0.0.4.4+0,3.2;0:32.512.256.128.16.32.64.32.2.2.8.16.2.0.0.4+0,3.2;1:32.512.256.128.16.32.64.32.0.4.8.16.2.0.2.4+0,2.2;1:32.512.256.128.16.32.64.32.2.4.8.16.0.0.4.4+2,3.2;1:32.512.256.128.16.32.64.32.2.4.8.16.0.0.2.8+2,3.4;3:32.512.256.128.16.32.64.32.2.4.8.16.2.8.4.0+0,3.2;0:32.512.256.128.16.32.64.32.4.4.8.16.2.8.4.0+3,2.2;3:32.512.256.128.16.32.64.32.8.8.16.2.2.8.4.0+3,3.2;3:32.512.256.128.16.32.64.32.16.16.2.0.2.8.4.2+0,3.2;0:32.512.256.128.32.32.64.32.2.16.2.2.2.8.4.0+0,3.2;0:64.512.256.128.4.32.64.32.0.16.2.2.2.8.4.0+0,3.2;1:64.512.256.128.4.32.64.32.0.0.16.4.2.2.8.4+3,3.2;0:64.512.256.128.4.32.64.32.2.2.16.8.0.0.8.2+3,2.4;3:64.512.256.128.4.32.64.32.4.16.8.4.8.2.0.0+0,3.2;0:64.512.256.128.8.32.64.32.8.16.8.4.2.2.0.0+2,3.2;0:64.512.256.128.16.32.64.32.2.16.8.4.0.2.2.0+2,3.2;3:64.512.256.128.16.32.64.32.2.16.8.4.4.0.2.0+1,3.2;1:64.512.256.128.16.32.64.32.2.16.8.4.0.2.4.2+3,3.2;3:64.512.256.128.16.32.64.32.2.16.8.4.2.4.2.2+0,3.2;0:64.512.256.128.16.32.64.32.4.16.8.4.2.4.2.2+0,3.2;1:64.512.256.128.16.32.64.32.4.16.8.4.2.2.4.4+3,3.2;0:64.512.256.128.16.32.64.32.4.16.8.8.2.2.4.2+3,2.2;3:64.512.256.128.16.32.64.32.4.16.16.2.4.4.2.0+3,3.2;0:64.512.256.128.16.32.64.32.8.16.16.2.0.4.2.2+3,3.2;3:64.512.256.128.16.32.64.32.8.32.2.0.4.4.0.2+1,3.2;0:64.512.256.128.16.64.64.32.8.4.2.2.4.2.0.0+0,2.2;1:64.512.256.128.0.16.128.32.2.8.4.4.0.0.4.2+0,3.2;1:64.512.256.128.0.16.128.32.0.2.8.8.2.0.4.2+1,2.2;1:64.512.256.128.0.16.128.32.0.2.2.16.0.2.4.2+1,2.4;1:64.512.256.128.0.16.128.32.0.4.4.16.0.2.4.2+1,2.2;1:64.512.256.128.0.16.128.32.0.2.8.16.0.2.4.2+0,3.2;0:64.512.256.128.0.16.128.32.0.4.8.16.2.0.4.2+0,3.2;1:64.512.256.128.0.16.128.32.0.4.8.16.2.2.4.2+0,2.2;1:64.512.256.128.0.16.128.32.2.4.8.16.0.4.4.2+0,3.2;0:64.512.256.128.2.16.128.32.0.8.8.16.2.0.4.2+0,2.2;1:64.512.256.128.2.16.128.32.2.0.16.16.0.2.4.2+0,2.2;1:64.512.256.128.2.16.128.32.2.0.2.32.0.2.4.2+3,3.2;0:64.512.256.128.4.16.128.64.0.2.2.2.0.0.4.2+0,3.4;1:64.512.256.128.4.16.128.64.0.0.2.4.4.0.4.2+1,2.2;0:64.512.256.128.8.16.128.64.0.2.2.4.0.0.4.2+1,2.4;1:64.512.256.128.8.16.128.64.0.4.4.4.0.0.4.2+0,2.2;1:64.512.256.128.8.16.128.64.2.0.4.8.0.0.4.2+0,3.2;0:64.512.256.128.8.16.128.64.2.0.8.8.2.0.0.2+2,3.2;1:64.512.256.128.8.16.128.64.0.0.2.16.0.0.2.4+0,3.2;0:64.512.256.128.8.16.128.64.0.0.4.16.2.0.0.4+1,2.4;1:64.512.256.128.8.16.128.64.0.4.4.16.0.0.2.4+0,2.2;1:64.512.256.128.8.16.128.64.2.0.8.16.0.0.2.4+1,3.4;1:64.512.256.128.8.16.128.64.0.2.8.16.0.4.2.4+3,3.2;3:64.512.256.128.8.16.128.64.2.8.16.0.4.2.4.2+3,3.2;0:64.512.256.128.8.16.128.64.2.8.16.2.4.2.4.2+3,3.2;f";
}
