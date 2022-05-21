#![feature(doc_auto_cfg)]
//! a basic engine and move validator for the game 2048
//! 
//! Includes wasm functions generated with wasm_bindgen

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
    let parsed = parser::parse_data(String::from(data)).unwrap();
    let out = parsed.history.iter().map(|x| {
        let board = board::Board{ width: parsed.width, height: parsed.height, tiles: x.0 };
        board.oispahalla_serialize()
    }).collect::<Vec<String>>();
    return serde_json::to_string(&out).unwrap();
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn validate(data: &str) -> String {
    let parsed = parser::parse_data(String::from(data)).unwrap();
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

#[cfg(feature = "add_random")]
use rand::prelude::SliceRandom;

#[cfg(feature = "add_random")]
pub fn add_random_to_board(board: &mut board::Board) {
    let mut possible = board.get_non_occupied_tiles();
    if possible.len() > 0 {
        possible.shuffle(&mut rand::thread_rng());
        let t = possible[0];
        let mut possible_values = vec![2, 2, 2, 4];
        possible_values.shuffle(&mut rand::thread_rng());
        board.set_tile(t.x, t.y, possible_values[0]);
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn add_random(board_data: &str) -> String {
    let b: board::Board = serde_json::from_str(board_data).unwrap();
    let mut game = board::Board{ width: b.width, height: b.height, tiles: b.tiles };
    add_random_to_board(&mut game);
    return serde_json::to_string(&game.tiles).unwrap();
}

#[cfg(all(feature = "wasm", feature = "history_hash"))]
#[wasm_bindgen]
pub fn hash(data: &str) -> String {
    let parsed = parser::parse_data(String::from(data)).unwrap();
    return serde_json::to_string(&parsed.hash_v1()).unwrap();
}

// Tests
#[cfg(test)]
pub mod tests;