//! Wasm bindings for the [unified](crate::unified) module
//!
//! This module is only available when the `wasm` feature is enabled

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{
    board::Board,
    direction::Direction,
    unified::{
        game::GameState, reconstruction::HistoryReconstruction, validation::ValidationResult,
        ParseResult,
    },
    v1::{recording::Recording, validator::initialize_board},
    v2::recording::SeededRecording,
    *,
};

fn err_str(e: impl std::fmt::Display) -> JsValue {
    JsValue::from_str(&format!("{}", e))
}

#[wasm_bindgen]
pub fn deserialize(data: &str) -> Result<ParseResult, JsValue> {
    unified::parse(data).map_err(err_str)
}

#[wasm_bindgen]
pub fn serialize(data: ParseResult) -> Result<String, JsValue> {
    match data {
        ParseResult::V1(rec) => Ok(format!("{}", rec)),
        ParseResult::V2(sedrec) => Ok(String::from(&sedrec)),
    }
}

#[wasm_bindgen]
pub fn reconstruct(data: &str) -> Result<HistoryReconstruction, JsValue> {
    unified::reconstruct(data).map_err(err_str)
}

#[wasm_bindgen]
pub fn validate(data: &str) -> Result<ValidationResult, JsValue> {
    unified::validate(data).map_err(err_str)
}

#[tsify::declare]
type ValidationResultOrError = Result<ValidationResult, String>;

#[tsify::declare]
type CompleteValidationResult = Result<Vec<ValidationResultOrError>, String>;

#[wasm_bindgen]
pub fn validate_all(data: &str) -> String {
    let result: CompleteValidationResult =
        unified::parse(data)
            .map_err(|e| e.to_string())
            .map(|parsed| {
                let mut results = Vec::new();
                match parsed {
                    ParseResult::V1(rec) => {
                        let mut moves_until_now = Vec::new();
                        for frame in rec.history {
                            let history_until_now = Recording {
                                width: rec.width,
                                height: rec.height,
                                history: moves_until_now.clone(),
                            };
                            results.push(
                                match unified::validate(&format!("{}", history_until_now)) {
                                    Ok(result) => ValidationResultOrError::Ok(result),
                                    Err(e) => ValidationResultOrError::Err(e.to_string()),
                                },
                            );
                            moves_until_now.push(frame);
                        }
                    }
                    ParseResult::V2(sedrec) => {
                        let mut moves_until_now = Vec::new();
                        for frame in sedrec.moves {
                            let history_until_now = SeededRecording {
                                version: sedrec.version,
                                width: sedrec.width,
                                height: sedrec.height,
                                seed: sedrec.seed,
                                moves: moves_until_now.clone(),
                            };
                            results.push(
                                match unified::validate(&String::from(&history_until_now)) {
                                    Ok(result) => ValidationResultOrError::Ok(result),
                                    Err(e) => ValidationResultOrError::Err(e.to_string()),
                                },
                            );
                            moves_until_now.push(frame);
                        }
                    }
                }
                results.push(match unified::validate(data) {
                    Ok(result) => ValidationResultOrError::Ok(result),
                    Err(e) => ValidationResultOrError::Err(e.to_string()),
                });
                results
            });
    serde_json::to_string(&result).unwrap()
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg(feature = "wasm")]
#[derive(tsify::Tsify)]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct MoveResult {
    board: Board,
    score_gain: usize,
}

#[wasm_bindgen]
pub fn initial_board(size: usize, seed: u32, add_tiles: usize) -> Board {
    initialize_board(size, size, seed, add_tiles)
}

#[wasm_bindgen]
pub fn new_game(size: usize, seed: Option<u32>) -> String {
    let seed = seed.unwrap_or_else(|| rand::random());
    String::from(&SeededRecording::empty(seed, size, size))
}

#[wasm_bindgen]
pub fn get_gamestate(data: &str) -> Result<GameState, JsValue> {
    unified::get_gamestate(data).map_err(err_str)
}

#[wasm_bindgen]
pub fn apply_move(board: Board, dir: Direction, add_random: bool) -> Result<MoveResult, JsValue> {
    let mut board: Board = board;
    let result = board.move_in_direction(dir);
    if result.is_ok() && add_random {
        board.add_random_tile();
    }
    result
        .map(|score_gain| MoveResult { board, score_gain })
        .map_err(err_str)
}

#[wasm_bindgen]
pub fn add_random(board: Board) -> Board {
    let mut board = board;
    board.add_random_tile();
    board
}

#[wasm_bindgen]
pub fn hash(data: &str) -> Result<String, JsValue> {
    unified::hash(data).map_err(err_str)
}

#[wasm_bindgen]
pub fn lcg_sane(seed: u32) -> u32 {
    let mut seed = seed;
    random::lcg_sane(&mut seed);
    seed
}
