use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::recording::SeededRecording;
use crate::{
    add_random_to_board,
    board::{check_move, Board, MoveError},
    direction::Direction,
    rules::Ruleset,
    v1::validator::{initialize_board, HistoryReconstruction, ValidationData, MAX_ALLOWED_BREAKS},
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
pub fn replay_moves(
    input: &SeededRecording,
    rules: Box<dyn Ruleset>,
) -> Result<HistoryReconstruction, MoveReplayError> {
    let mut score: usize = 0;
    let mut max_score: usize = 0;

    let mut board = initialize_board(input.width, input.height, input.seed, 2);
    let mut history_out: Vec<Board> = vec![board];

    let breaks: usize = 0;
    let break_positions = [None; MAX_ALLOWED_BREAKS];

    for (move_index, mv) in input.moves.iter().copied().enumerate() {
        if mv == Direction::BREAK {
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
            let tile_threshold = rules.break_tile_threshold(&board);
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
        let mvchk =
            check_move(board, mv).map_err(|e| MoveReplayError::InvalidMove(mv, move_index, e))?;

        board = mvchk.board;
        score += mvchk.score_gain;
        max_score = usize::max(score, max_score);

        add_random_to_board(&mut board);

        history_out.push(board);
    }

    Ok(HistoryReconstruction {
        validation_data: ValidationData {
            valid: true,
            score: max_score,
            score_end: score,
            score_margin: 0,
            breaks,
            break_positions,
        },
        history: history_out,
    })
}
