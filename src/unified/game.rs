use serde::{Deserialize, Serialize};

use crate::{
    board::{check_move, Board},
    direction::{self, Direction},
    rules::RulesetProvider,
};

use super::reconstruction::Reconstructable;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg(feature = "wasm")]
#[derive(tsify::Tsify)]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct GameState {
    pub board: Board,
    pub score: usize,
    pub breaks: usize,
    pub allowed_moves: Vec<Direction>,
    pub over: bool,
    pub won: bool,
}
impl GameState {
    pub fn from_reconstructable_ruleset<T: Reconstructable + RulesetProvider>(
        reconstruction: &T,
    ) -> Result<Self, T::ReconstructionError> {
        let rules = reconstruction.rules();
        let reconstruction = reconstruction.reconstruct()?;
        let score = reconstruction.validation_data.score;
        let breaks = reconstruction.validation_data.breaks;
        let board = *reconstruction.history.last().unwrap();
        let mut allowed_moves = vec![];
        for direction in direction::MOVE_DIRECTIONS {
            if check_move(board, direction).is_ok() {
                allowed_moves.push(direction);
            }
        }
        let allowed_to_break = crate::rules::can_break(rules, &board, score, breaks);
        if allowed_to_break {
            allowed_moves.push(Direction::BREAK);
        }

        let over = allowed_moves.is_empty();
        let won = rules.won(&board);
        Ok(Self {
            board,
            score,
            breaks,
            allowed_moves,
            over,
            won,
        })
    }
}
