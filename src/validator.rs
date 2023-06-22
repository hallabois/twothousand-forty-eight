//! Provides functions to validate a [Recording](crate::recording::Recording)
use crate::add_random_to_board;
use crate::board::check_move;
use crate::board::tile::Tile;
use crate::board::Board;
use crate::direction;
use crate::direction::Direction;
use crate::parser::ParseDataV2;
use crate::recording::Recording;

use serde::Deserialize;
use serde::Serialize;

/// the validator will stop validating history beyond [MAX_HISTORY_LENGTH]
pub const MAX_HISTORY_LENGTH: usize = usize::MAX;

pub const MAX_ALLOWED_BREAKS: usize = 3;

use thiserror::Error;
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ValidationError {
    #[error("invalid addition `{1:?}` on move {0}")]
    InvalidAddition(usize, Tile),

    #[error("invalid score on move {0}, expected `{1}` but got `{2}`")]
    InvalidScore(usize, usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ValidationData {
    /// Depreciated, will always be true, specific errors are now returned on invalid games
    pub valid: bool,
    /// The maximum score reached during the run, 0 if the run is not valid
    pub score: usize,
    /// The score at the end of the run, may not match score if a break was used near the end of the run
    pub score_end: usize,
    /// Error margin on the score, in case the last move was not recorded
    pub score_margin: usize,
    /// Amount of breaks used
    pub breaks: usize,
    /// When those breaks happened
    pub break_positions: [Option<usize>; MAX_ALLOWED_BREAKS],
}

/// Validates the history continuity and returns the determined validity, score, possible score margin (caused when the last game move was not recorded) and break count.
pub fn validate_history(history: Recording) -> Result<ValidationData, ValidationError> {
    let reconstruction = reconstruct_history(history)?;
    Ok(reconstruction.validation_data)
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub struct HistoryReconstruction {
    pub validation_data: ValidationData,
    pub history: Vec<Board>,
}
pub fn reconstruct_history(history: Recording) -> Result<HistoryReconstruction, ValidationError> {
    let mut score: usize = 0;
    let mut score_margin: usize = 0;
    let mut max_score: usize = 0;

    let history_len = history.history.len();
    let mut history_out: Vec<Board> = vec![];
    if history_len > 0 {
        // Push the starting position into history, as it is not validated here.
        history_out.push(Board {
            height: history.height,
            width: history.width,
            tiles: history.history[0].0,
            ..Default::default()
        });
    }

    let mut breaks: usize = 0;
    let mut break_positions = [None; MAX_ALLOWED_BREAKS];
    for ind in 0..history_len {
        let i = history.history[ind];

        let board = i.0;
        let dir = i.1;
        let addition = history.history[ind].2;

        let tiles = if ind > 0 {
            history_out[ind].tiles
        } else {
            board
        };
        let board_to_check = Board {
            tiles,
            width: history.width,
            height: history.height,
            ..Default::default()
        };
        let predicted = check_move(board_to_check, dir);
        let mut predicted_board = tiles;
        if let Ok(data) = predicted {
            predicted_board = data.board.tiles;
            score += data.score_gain;
            max_score = usize::max(score, max_score);
        }

        if ind < (history_len - 1) && ind < MAX_HISTORY_LENGTH {
            let board_next = history.history[ind + 1].0;
            if let Some(add) = addition {
                if add.value > 4 {
                    println!("Invalid addition value at {:?}!", add);
                    return Err(ValidationError::InvalidAddition(ind, add));
                };
                predicted_board[add.y][add.x] = Some(add);
            }

            let mut board_predicted = Board {
                tiles: predicted_board,
                width: history.width,
                height: history.height,
                ..Default::default()
            };
            let board_actual = Board {
                tiles: board_next,
                width: history.width,
                height: history.height,
                ..Default::default()
            };
            let expected_score = board_predicted.get_total_value();
            let actual_score = board_actual.get_total_value();

            let ended_correctly = dir == Direction::END && expected_score == actual_score;
            let continued_correctly = predicted_board == board_next;
            if ended_correctly || continued_correctly {
                // Do nothing, the move was valid
            } else if breaks < 3 && (expected_score > actual_score) && score > 999 {
                // Kurinpalautus / Parinkulautus
                break_positions[breaks] = Some(ind);
                breaks += 1;
                score -= 1000;

                // Clear tiles with value < 16
                board_predicted.tiles = board_predicted.tiles.map(|c| {
                    c.map(|t| {
                        t.map(|t| {
                            if t.value >= 16 {
                                t
                            } else {
                                Tile { value: 0, ..t }
                            }
                        })
                    })
                });
            } else {
                // Invalid move
                println!(
                    "Went wrong at index {} (move to {:?}): \n{:?}\n{:?}",
                    ind, dir, predicted_board, board_next
                );
                if ind > 0 {
                    println!("Last board:");
                    println!("{}", Board::from(history_out.last().unwrap().tiles));
                }
                println!("Expected: (score {}) ", expected_score);
                println!("{}", Board::from(predicted_board));
                println!("Got instead: (score {}) ", actual_score);
                println!("{}", Board::from(board_next));
                return Err(ValidationError::InvalidScore(
                    ind,
                    expected_score,
                    actual_score,
                ));
            }
            history_out.push(board_predicted);
        }
    }
    // Get the score margin
    let last_history = history.history.last();
    match last_history {
        None => {}
        Some(last_history) => {
            let last_board = last_history.0;
            for dir in direction::REAL_DIRECTIONS {
                let predicted = check_move(
                    Board {
                        tiles: last_board,
                        width: history.width,
                        height: history.height,
                        ..Default::default()
                    },
                    dir,
                );
                if let Ok(data) = predicted {
                    score_margin = score_margin.max(data.score_gain);
                }
            }
        }
    }

    Ok(HistoryReconstruction {
        validation_data: ValidationData {
            valid: true,
            score: max_score,
            score_end: score,
            score_margin,
            breaks,
            break_positions,
        },
        history: history_out,
    })
}

/// Makes sure that the starting state of the history doesn't contain too many tiles or tiles with a too big value.
pub fn validate_first_move(history: &Recording) -> bool {
    let history_len = history.history.len();
    if history_len > 0 {
        let first_frame = history.history[0].0;
        let first_board = Board {
            tiles: first_frame,
            width: history.width,
            height: history.height,
            ..Default::default()
        };
        if first_board.get_total_value() < 17 {
            return true;
        }
    }
    false
}

/// Returns the accumulated score of a run (should match the score displayed in the game)
pub fn get_run_score(history: &Recording) -> usize {
    let mut score: usize = 0;
    for i in &history.history {
        let board = i.0;
        let dir = i.1;
        let predicted = check_move(
            Board {
                tiles: board,
                width: history.width,
                height: history.height,
                ..Default::default()
            },
            dir,
        );
        if let Ok(data) = predicted {
            score += data.score_gain;
        }
        // should we return the intermediate score if a move is invalid?
    }
    score
}

pub fn initialize_board(width: usize, height: usize, seed: usize, add_tiles: usize) -> Board {
    let mut board = Board::new(
        width,
        height,
        crate::board::tile_id_assigner::IDAssignment::default(),
        Some(seed),
    );
    for _ in 0..add_tiles {
        crate::add_random_to_board(&mut board);
    }

    board
}

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum MoveReplayError {
    #[error("invalid move `{0:?}` on move {1}")]
    InvalidMove(Direction, usize),

    #[error("can't break on move {0} as {1}/{2} breaks have already been used")]
    InvalidBreak(usize, usize, usize),
}

/// Intended for reconstructing V2 format games
pub fn replay_moves(input: ParseDataV2) -> Result<HistoryReconstruction, MoveReplayError> {
    let mut score: usize = 0;
    let mut max_score: usize = 0;

    let mut board = initialize_board(input.width, input.height, input.seed, 2);
    let mut history_out: Vec<Board> = vec![board];

    let breaks: usize = 0;
    let break_positions = [None; MAX_ALLOWED_BREAKS];

    for (move_index, mv) in input.moves.into_iter().enumerate() {
        let mvchk =
            check_move(board, mv).map_err(|_| MoveReplayError::InvalidMove(mv, move_index))?;

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
