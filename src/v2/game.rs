use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::recording::SeededRecording;
use crate::{
    add_random_to_board,
    board::{check_move, Board, MoveError},
    direction::{self, Direction},
    rules::{self, Ruleset},
    v1::validator::{initialize_board, HistoryReconstruction, ValidationData, MAX_ALLOWED_BREAKS},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub board: Board,
    pub score: usize,
    pub breaks: usize,
    pub break_positions: [Option<usize>; MAX_ALLOWED_BREAKS],
    pub allowed_moves: Vec<Direction>,
    pub over: bool,
    pub won: bool,
    pub history: SeededRecording,
}

impl TryFrom<&SeededRecording> for GameState {
    type Error = MoveReplayError;

    fn try_from(history: &SeededRecording) -> Result<Self, Self::Error> {
        let rules = history.get_ruleset();
        let replay = replay_moves(history, rules.as_ref())?;

        let score = replay.validation_data.score;
        let breaks = replay.validation_data.breaks;
        let break_positions = replay.validation_data.break_positions;
        let board = match replay.history.last() {
            Some(board) => *board,
            None => initialize_board(history.width, history.height, history.seed, 2),
        };

        let mut allowed_moves = vec![];
        for direction in direction::MOVE_DIRECTIONS {
            if check_move(board, direction).is_ok() {
                allowed_moves.push(direction);
            }
        }
        let allowed_to_break = rules::can_break(rules.as_ref(), &board, score, breaks);
        if allowed_to_break {
            allowed_moves.push(Direction::BREAK);
        }

        let over = allowed_moves.is_empty();
        let won = rules.won(&board);
        Ok(GameState {
            board,
            score,
            breaks,
            break_positions,
            allowed_moves,
            over,
            won,
            history: history.clone(),
        })
    }
}

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
    rules: &dyn Ruleset,
) -> Result<HistoryReconstruction, MoveReplayError> {
    let mut score: usize = 0;
    let mut max_score: usize = 0;

    let mut board = initialize_board(input.width, input.height, input.seed, 2);
    let mut history_out: Vec<Board> = vec![board];

    let mut breaks: usize = 0;
    let mut break_positions = [None; MAX_ALLOWED_BREAKS];

    for (move_index, mv) in input.moves.iter().copied().enumerate() {
        if mv != Direction::BREAK {
            let mvchk = check_move(board, mv)
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

fn actuate_break(board: &mut Board, rules: &dyn Ruleset) {
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
