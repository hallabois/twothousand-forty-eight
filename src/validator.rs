//! Provides functions to validate a [Recording](crate::recording::Recording)
use crate::board::is_move_possible;
use crate::board::print_board;
use crate::board::tile::Tile;
use crate::board::Board;
use crate::direction;
use crate::direction::Direction;
use crate::recording::Recording;

#[cfg(feature = "wasm")]
use serde::Deserialize;
#[cfg(feature = "wasm")]
use serde::Serialize;

/// the validator will stop validating history beyond [MAX_HISTORY_LENGTH]
const MAX_HISTORY_LENGTH: usize = usize::MAX;

use thiserror::Error;
#[derive(Error, Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub enum ValidationError {
    #[error("invalid addition `{1:?}` on move {0}")]
    InvalidAddition(usize, Tile),

    #[error("invalid score on move {0}, expected `{1}` but got `{2}`")]
    InvalidScore(usize, usize, usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
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
}

/// Validates the history continuity and returns the determined validity, score, possible score margin (caused when the last game move was not recorded) and break count.
pub fn validate_history(history: Recording) -> Result<ValidationData, ValidationError> {
    let mut score: usize = 0;
    let mut score_margin: usize = 0;
    let mut max_score: usize = 0;

    let history_len = history.history.len();
    let mut breaks: usize = 0;
    for ind in 0..history_len {
        let i = history.history[ind];

        let board = i.0;
        let dir = i.1;
        let addition = history.history[ind].2;

        let predicted = is_move_possible(
            Board {
                tiles: board,
                width: history.width,
                height: history.height,
            },
            dir,
        );
        let mut predicted_board = predicted.0;
        score += predicted.2;
        max_score = usize::max(score, max_score);

        if ind < (history_len - 1) && ind < MAX_HISTORY_LENGTH {
            let board_next = history.history[ind + 1].0;
            match addition {
                Some(add) => {
                    if crate::DEBUG_INFO {
                        println!(
                            "[Add] Change {:?} => {:?}",
                            predicted_board[add.y][add.x], add
                        )
                    };
                    if add.value > 4 {
                        println!("Invalid addition value at {:?}!", add);
                        return Err(ValidationError::InvalidAddition(ind, add));
                    };
                    predicted_board[add.y][add.x] = Some(add);
                }
                None => {
                    if crate::DEBUG_INFO {
                        println!("No addition at index {}!", ind)
                    };
                }
            }

            let board_predicted = Board {
                tiles: predicted_board,
                width: history.width,
                height: history.height,
            };
            let board_actual = Board {
                tiles: board_next,
                width: history.width,
                height: history.height,
            };
            let expected_score = board_predicted.get_total_value();
            let actual_score = board_actual.get_total_value();
            if dir == Direction::END && expected_score == actual_score {
            } else if predicted_board == board_next { // (predicted.1) &&
            } else if breaks < 3 && (expected_score > actual_score) && score > 999 {
                //Kurinpalautus / Parinkulautus
                breaks += 1;
                score -= 1000;
            } else {
                println!(
                    "Went wrong at index {}: \n{:?}\n{:?}",
                    ind, predicted_board, board_next
                );
                //println!("{:#?}", i);
                println!("Expected: (score {}) ", expected_score);
                print_board(predicted_board, history.width, history.height);
                println!("Got instead: (score {}) ", actual_score);
                print_board(board_next, history.width, history.height);
                return Err(ValidationError::InvalidScore(
                    ind,
                    expected_score,
                    actual_score,
                ));
            }
        }
    }
    // Get the score margin
    let last_history = history.history.last();
    match last_history {
        None => {}
        Some(last_history) => {
            let last_board = last_history.0;
            for dir in direction::REAL_DIRECTIONS {
                let predicted = is_move_possible(
                    Board {
                        tiles: last_board,
                        width: history.width,
                        height: history.height,
                    },
                    dir,
                );
                score_margin = score_margin.max(predicted.2);
            }
        }
    }

    return Ok(ValidationData {
        valid: true,
        score: max_score,
        score_end: score,
        score_margin,
        breaks,
    });
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
        };
        if first_board.get_total_value() < 17 {
            return true;
        }
    }
    println!("First move was not OK!");
    return false;
}

/// Returns the accumulated score of a run (should match the score displayed in the game)
pub fn get_run_score(history: &Recording) -> usize {
    let mut score: usize = 0;
    for i in &history.history {
        let board = i.0;
        let dir = i.1;
        let predicted = is_move_possible(
            Board {
                tiles: board,
                width: history.width,
                height: history.height,
            },
            dir,
        );
        score += predicted.2;
    }
    score
}
