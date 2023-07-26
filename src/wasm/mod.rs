//! Wasm bindings for the [unified](crate::unified) module
//!
//! This module is only available when the `wasm` feature is enabled

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{board::Board, *};

fn err_str(e: impl std::fmt::Display) -> String {
    format!("{}", e)
}

#[wasm_bindgen]
pub fn parse(data: &str) -> Result<JsValue, JsValue> {
    let result = unified::parse(data).map_err(err_str)?;
    Ok(serde_wasm_bindgen::to_value(&result)?)
}

#[wasm_bindgen]
pub fn get_frames(data: &str) -> Result<JsValue, JsValue> {
    let result = unified::reconstruct(data).map_err(err_str)?;
    Ok(serde_wasm_bindgen::to_value(&result)?)
}

#[wasm_bindgen]
pub fn validate(data: &str) -> Result<JsValue, JsValue> {
    let result = unified::validate(data).map_err(err_str)?;
    Ok(serde_wasm_bindgen::to_value(&result)?)
}
/*
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
*/

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WasmMoveResult {
    board: Board,
    score_gain: usize,
}

#[wasm_bindgen]
pub fn apply_move(board_data: &str, dir: usize, add_random: bool) -> Result<JsValue, JsValue> {
    let mut board: Board = serde_json::from_str(board_data).map_err(err_str)?;
    let result = board.move_in_direction(direction::Direction::from_index(dir));
    if result.is_ok() && add_random {
        board.add_random_tile();
    }
    Ok(serde_wasm_bindgen::to_value(
        &result.map(|score_gain| WasmMoveResult { board, score_gain }),
    )?)
}

#[wasm_bindgen]
pub fn add_random(board_data: &str) -> String {
    let mut game: Board = serde_json::from_str(board_data).unwrap();
    game.add_random_tile();
    serde_json::to_string(&game.tiles).unwrap()
}

#[wasm_bindgen]
pub fn hash(data: &str) -> Result<JsValue, JsValue> {
    let result = unified::hash(data).map_err(err_str)?;
    Ok(serde_wasm_bindgen::to_value(&result)?)
}
