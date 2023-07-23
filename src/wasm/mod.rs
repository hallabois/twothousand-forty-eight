//! Wasm bindings for common functions
//!
//! This module is only available when the `wasm` feature is enabled

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{board::Board, *};

#[wasm_bindgen]
pub fn parse(data: &str) -> String {
    let parsed = v1::parser::parse_data(data);
    serde_json::to_string(&parsed).unwrap()
}

#[wasm_bindgen]
pub fn get_frames(data: &str) -> String {
    let parsed = v1::parser::parse_data(data).unwrap();
    let reconstruction = v1::validator::reconstruct_history(&parsed).unwrap();

    serde_json::to_string(&reconstruction.history).unwrap()
}

#[wasm_bindgen]
pub fn validate(data: &str) -> String {
    let parsed = v1::parser::parse_data(data).unwrap();
    // let first_move_valid = validator::validate_first_move(&parsed);
    let history_valid = v1::validator::validate_history(parsed);
    serde_json::to_string(&history_valid).unwrap()
}

#[wasm_bindgen]
pub fn validate_all_frames(data: &str) -> String {
    let frames_src = data.split(':').collect::<Vec<&str>>();
    let frame_count = frames_src.clone().len();
    println!("found {} frames", frame_count);
    let mut validation_results: Vec<
        Option<Result<v1::validator::ValidationData, v1::validator::ValidationError>>,
    > = vec![];

    for frame in 0..frame_count {
        let section = frames_src[0..frame].join(":");
        match v1::parser::parse_data(&section) {
            Ok(parsed) => {
                let history_valid = v1::validator::validate_history(parsed);
                validation_results.push(Some(history_valid));
            }
            Err(_) => {
                validation_results.push(None);
            }
        }
    }
    serde_json::to_string(&validation_results).unwrap()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WasmMoveResult {
    board: Board,
    score_gain: usize,
}

#[wasm_bindgen]
pub fn apply_move(board_data: &str, dir: usize, add_random: bool) -> String {
    let mut board: Board = serde_json::from_str(board_data).unwrap();
    let result = board.move_in_direction(direction::Direction::from_index(dir));
    if result.is_ok() && add_random {
        crate::add_random_to_board(&mut board);
    }
    serde_json::to_string(&result.map(|score_gain| WasmMoveResult { board, score_gain })).unwrap()
}

#[wasm_bindgen]
pub fn add_random(board_data: &str) -> String {
    let mut game: Board = serde_json::from_str(board_data).unwrap();
    add_random_to_board(&mut game);
    serde_json::to_string(&game.tiles).unwrap()
}

#[wasm_bindgen]
pub fn hash(data: &str) -> String {
    let parsed = v1::parser::parse_data(data).unwrap();
    serde_json::to_string(&parsed.hash_v1()).unwrap()
}
