pub mod board;
pub mod parser;
pub mod validator;
pub mod direction;
pub mod recording;

pub const DEBUG_INFO: bool = false;


#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
use rand::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse(data: &str) -> String {
    let parsed = parser::parse_data(String::from(data));
    return serde_json::to_string(&parsed).unwrap();
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn get_frames(data: &str) -> String {
    let parsed = parser::parse_data(String::from(data));
    let out = parsed.history.iter().map(|x| {
        let board = board::Board{ width: parsed.width, height: parsed.height, tiles: x.0 };
        board.oispahalla_serialize()
    }).collect::<Vec<String>>();
    return serde_json::to_string(&out).unwrap();
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn validate(data: &str) -> String {
    let parsed = parser::parse_data(String::from(data));
    // let first_move_valid = validator::validate_first_move(&parsed);
    let history_valid = validator::validate_history(parsed);
    return serde_json::to_string(&history_valid).unwrap();
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn apply_move(board_data: &str, dir: usize, add_random: bool) -> String {
    let b = serde_json::from_str(board_data).unwrap();
    // let first_move_valid = validator::validate_first_move(&parsed);
    let mut result = board::is_move_possible(b, direction::Direction::from_index(dir));
    if add_random {
        let mut game = board::Board{ width: b.width, height: b.height, tiles: result.0 };
        let mut possible = game.get_non_occupied_tiles();
        if possible.len() > 0 {
            possible.shuffle(&mut rand::thread_rng());
            let t = possible[0];
            let mut possible_values = vec![2, 2, 2, 4];
            possible_values.shuffle(&mut rand::thread_rng());
            game.set_tile(t.x, t.y, possible_values[0]);
        }
        result.0 = game.tiles;
    }
    return serde_json::to_string(&result).unwrap();
}


#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn add_random(board_data: &str) -> String {
    let b: board::Board = serde_json::from_str(board_data).unwrap();
    let mut game = board::Board{ width: b.width, height: b.height, tiles: b.tiles };
    let mut possible = game.get_non_occupied_tiles();
    if possible.len() > 0 {
        possible.shuffle(&mut rand::thread_rng());
        let t = possible[0];
        let mut possible_values = vec![2, 2, 2, 4];
        possible_values.shuffle(&mut rand::thread_rng());
        game.set_tile(t.x, t.y, possible_values[0]);
    }
    return serde_json::to_string(&game.tiles).unwrap();
}

#[cfg(all(feature = "wasm", feature = "history_hash"))]
#[wasm_bindgen]
pub fn hash(data: &str) -> String {
    let parsed = parser::parse_data(String::from(data));
    return serde_json::to_string(&parsed.hash_v1()).unwrap();
}

// Tests
pub mod lib_testgames;

#[cfg(test)]
mod tests {

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

    mod parser {
        use crate::lib_testgames;
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

    mod validator {
        use crate::lib_testgames;
        use crate::parser;
        use crate::validator;

        pub fn assert_score(score: usize, expected: usize, score_margin: usize) {
            assert!(score <= expected+score_margin, "{} !< {}", score, expected+score_margin);
        }

        #[test]
        fn validator_works_normal_4x4_0breaks_a() {
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
        fn validator_works_normal_4x4_0breaks_b() {
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
        fn validator_works_normal_4x4_2breaks() {
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
        fn validator_works_normal_3x3_0breaks_a() {
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
        fn validator_works_normal_3x3_0breaks_b() {
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
        fn validator_works_looong_4x4_0breaks() {
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

    #[cfg(feature = "history_hash")]
    mod history_hash {
        use crate::lib_testgames;
        use crate::parser;

        #[test]
        fn history_hash_works() {
            use lib_testgames::GAME4X4;
            let history = parser::parse_data(String::from(GAME4X4));
            assert_eq!(history.hash_v1(), String::from("9CAC2643E4E5F66E18FD9150320471F016CAF69FA3865A6DAE1DD9726F6792F5"));
        }
    }

}