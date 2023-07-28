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
            board.add_random_tile();
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
            if rules.game_over(&board) {
                return Err(MoveReplayError::InvalidMove(
                    mv,
                    move_index,
                    MoveError::NoValidMovesLeft,
                ));
            }
            score -= cost;
            break_positions[breaks] = Some(move_index);
            actuate_break(&mut board, rules);
            breaks += 1;
        }

        max_score = usize::max(score, max_score);
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

#[cfg(test)]
pub mod tests {
    use crate::{
        unified::game::GameState,
        v2::{
            recording::SeededRecording,
            test_data::{GAME_INVALID_BREAK_AFTER_LOSS, GAME_NI4FIRM, GAME_WON_3_BREAKS},
        },
    };

    #[test]
    fn correctness_a() {
        let rec: SeededRecording = GAME_NI4FIRM.parse().unwrap();
        let state = GameState::from_reconstructable_ruleset(&rec).unwrap();
        assert_eq!(state.board.width, 4);
        assert_eq!(state.board.height, 4);
        assert_eq!(state.score_current, 604);
        assert_eq!(state.score_max, 604);
        assert_eq!(state.breaks, 0);
        assert_eq!(state.won, false);
        assert_eq!(state.over, true);
    }

    #[test]
    fn correctness_b() {
        let rec: SeededRecording = GAME_WON_3_BREAKS.parse().unwrap();
        let state = GameState::from_reconstructable_ruleset(&rec).unwrap();
        assert_eq!(state.board.width, 4);
        assert_eq!(state.board.height, 4);
        assert_eq!(state.score_current, 16768);
        assert_eq!(state.score_max, 16768);
        assert_eq!(state.breaks, 3);
        assert_eq!(state.won, true);
        assert_eq!(state.over, false);
    }

    #[test]
    #[should_panic]
    fn correctness_c() {
        let rec: SeededRecording = GAME_INVALID_BREAK_AFTER_LOSS.parse().unwrap();
        let state = GameState::from_reconstructable_ruleset(&rec).unwrap();

        // not a valid run, should've panicked by now
        println!("{:?}", state);
    }
}
