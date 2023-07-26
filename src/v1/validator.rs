//! Provides functions to validate a [Recording](crate::v1::recording::Recording)
use crate::board::check_move;
use crate::board::tile::Tile;
use crate::board::Board;
use crate::direction;
use crate::direction::Direction;
use crate::rules::{ClassicV1, Ruleset, RulesetProvider};
use crate::unified::reconstruction::{HistoryReconstruction, Reconstructable};
use crate::unified::validation::{Validatable, ValidationResult, MAX_ALLOWED_BREAKS};
use crate::v1::recording::Recording;

/// the validator will stop validating history beyond [MAX_HISTORY_LENGTH]
const MAX_HISTORY_LENGTH: usize = usize::MAX;

use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ValidationError {
    #[error("invalid addition `{1:?}` on move {0}")]
    InvalidAddition(usize, Tile),

    #[error("invalid score on move {0}, expected `{1}` but got `{2}`")]
    InvalidScore(usize, usize, usize),
}

impl Validatable for Recording {
    type Error = ValidationError;
    fn validate(&self) -> Result<ValidationResult, Self::Error> {
        let reconstruction = self.reconstruct()?;
        Ok(reconstruction.validation_data)
    }
}

impl Reconstructable for Recording {
    type ReconstructionError = ValidationError;
    fn reconstruct(&self) -> Result<HistoryReconstruction, Self::ReconstructionError> {
        let mut score: usize = 0;
        let mut score_margin: usize = 0;
        let mut max_score: usize = 0;

        let mut rng_state = 0;
        let history_len = self.history.len();
        let mut history_out: Vec<Board> = vec![];
        if history_len > 0 {
            // Push the starting position into history, as it is not validated here.
            let board = Board::from((self.history[0].0, rng_state));
            rng_state = board.rng_state; // Update the rng state
            history_out.push(board);
        }

        let mut breaks: usize = 0;
        let mut break_positions = [None; MAX_ALLOWED_BREAKS];
        for ind in 0..history_len {
            let i = self.history[ind];

            let board = i.0;
            let dir = i.1;
            let addition = self.history[ind].2;

            let tiles = if ind > 0 {
                history_out[ind].tiles
            } else {
                board
            };
            let board_to_check = Board::from((tiles, rng_state));
            let predicted = check_move(board_to_check, dir);
            let mut predicted_board = tiles;
            if let Ok(data) = predicted {
                predicted_board = data.board.tiles;
                score += data.score_gain;
                max_score = usize::max(score, max_score);
            }

            if ind < (history_len - 1) && ind < MAX_HISTORY_LENGTH {
                let board_next = self.history[ind + 1].0;
                if let Some(add) = addition {
                    if add.value > 4 {
                        println!("Invalid addition value at {:?}!", add);
                        return Err(ValidationError::InvalidAddition(ind, add));
                    };
                    predicted_board[add.y][add.x] = Some(add);
                }

                let mut board_predicted = Board::from((predicted_board, board_to_check.rng_state));
                let board_actual = Board::from((board_next, board_to_check.rng_state));
                let expected_score = board_predicted.get_total_value();
                let actual_score = board_actual.get_total_value();

                let ended_correctly = dir == Direction::END && expected_score == actual_score;
                let continued_correctly = predicted_board == board_next;
                if ended_correctly || continued_correctly {
                    // Do nothing, the move was valid
                } else if (expected_score > actual_score)
                    && crate::rules::can_break(&ClassicV1, &board_predicted, score, breaks)
                {
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
                        println!(
                            "{}",
                            Board::from((history_out.last().unwrap().tiles, rng_state))
                        );
                    }
                    println!("Expected: (score {}) ", expected_score);
                    println!("{}", Board::from((predicted_board, rng_state)));
                    println!("Got instead: (score {}) ", actual_score);
                    println!("{}", Board::from((board_next, rng_state)));
                    return Err(ValidationError::InvalidScore(
                        ind,
                        expected_score,
                        actual_score,
                    ));
                }
                history_out.push(board_predicted);
            }
            rng_state = board_to_check.rng_state;
        }
        // Get the score margin
        let last_history = self.history.last();
        match last_history {
            None => {}
            Some(last_history) => {
                let last_board = last_history.0;
                for dir in direction::REAL_DIRECTIONS {
                    let board = Board::from((last_board, rng_state));
                    rng_state = board.rng_state;
                    let predicted = check_move(board, dir);
                    if let Ok(data) = predicted {
                        score_margin = score_margin.max(data.score_gain);
                    }
                }
            }
        }

        Ok(HistoryReconstruction {
            validation_data: ValidationResult {
                score: max_score,
                score_end: score,
                score_margin,
                breaks,
                break_positions,
            },
            history: history_out,
        })
    }
}

impl RulesetProvider for Recording {
    fn rules(&self) -> &dyn Ruleset {
        &ClassicV1
    }
}

/// Makes sure that the starting state of the history doesn't contain too many tiles or tiles with a too big value.
pub fn validate_first_move(history: &Recording) -> bool {
    let history_len = history.history.len();
    if history_len > 0 {
        let first_frame = history.history[0].0;
        let first_board = Board::from((first_frame, 0));
        if first_board.get_total_value() < 17 {
            return true;
        }
    }
    false
}

pub fn initialize_board(width: usize, height: usize, seed: usize, add_tiles: usize) -> Board {
    let mut board = Board::new(width, height, seed);
    for _ in 0..add_tiles {
        board.add_random_tile();
    }

    board
}
