use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::recording::SeededRecording;
use crate::rules::RulesetProvider;
use crate::unified::reconstruction::{HistoryReconstruction, Reconstructable};
use crate::unified::validation::{ValidationResult, MAX_ALLOWED_BREAKS};
use crate::{
    board::{Board, MoveError},
    direction::Direction,
    rules::Ruleset,
    v1::validator::initialize_board,
};

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum MoveReplayError {
    #[error("invalid move `{0:?}` on move {1}: {2:?}")]
    InvalidMove(Direction, usize, MoveError),

    #[error("can't break on move {0} as {1}/{2} breaks have already been used")]
    TooManyBreaks(usize, usize, usize),

    #[error("can't break on move {0} as score {1} is't high enough (min {2})")]
    NotEnoughScoreToBreak(usize, usize, usize),
}

/// Intended for reconstructing V2 format games
pub fn replay_moves(recording: &SeededRecording) -> Result<HistoryReconstruction, MoveReplayError> {
    let rules = recording.rules();
    let mut score: usize = 0;
    let mut max_score: usize = 0;

    let mut board = initialize_board(recording.width, recording.height, recording.seed, 2);
    let mut history_out: Vec<Board> = vec![board];

    let mut breaks: usize = 0;
    let mut break_positions = [None; MAX_ALLOWED_BREAKS];

    for (move_index, mv) in recording.moves.iter().copied().enumerate() {
        if mv != Direction::BREAK {
            let mvchk = crate::board::check_move(board, mv)
                .map_err(|e| MoveReplayError::InvalidMove(mv, move_index, e))?;
            board = mvchk.board;
            score += mvchk.score_gain;
        } else {
            // check if a break is allowed
            let max_breaks = rules.break_max(&board);
            if breaks >= max_breaks {
                return Err(MoveReplayError::TooManyBreaks(
                    move_index, breaks, max_breaks,
                ));
            }
            let cost = rules.break_cost(&board);
            if score < cost {
                return Err(MoveReplayError::NotEnoughScoreToBreak(
                    move_index, score, cost,
                ));
            }
            score -= cost;
            break_positions[breaks] = Some(move_index);
            actuate_break(&mut board, rules);
            breaks += 1;
        }

        max_score = usize::max(score, max_score);
        board.add_random_tile();
        history_out.push(board);
    }

    Ok(HistoryReconstruction {
        validation_data: ValidationResult {
            score: max_score,
            score_end: score,
            score_margin: 0,
            breaks,
            break_positions,
        },
        history: history_out,
    })
}

fn actuate_break(board: &mut Board, rules: &dyn Ruleset) {
    let tile_threshold = rules.break_tile_threshold(board);
    // remove all tiles with value < tile_threshold
    board.tiles = board.tiles.map(|c| {
        c.map(|t| {
            t.map(|t| {
                if t.value >= tile_threshold {
                    t
                } else {
                    crate::board::tile::Tile { value: 0, ..t }
                }
            })
        })
    });
}

impl Reconstructable for SeededRecording {
    type ReconstructionError = MoveReplayError;
    fn reconstruct(&self) -> Result<HistoryReconstruction, Self::ReconstructionError> {
        replay_moves(self)
    }
}
