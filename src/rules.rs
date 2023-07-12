//! Rulesets for the game.
//!
//! Note that these are only really usable with the [v2](crate::v2) interface.

use crate::board::Board;

pub trait Ruleset {
    fn break_cost(&self, board: &Board) -> usize;
    fn break_max(&self, board: &Board) -> usize;
    fn break_tile_threshold(&self, board: &Board) -> usize;
    fn game_over(&self, board: &Board) -> bool;
    fn won(&self, board: &Board) -> bool;
}

pub fn can_break(rules: &dyn Ruleset, board: &Board, score: usize, breaks: usize) -> bool {
    breaks < rules.break_max(board) && rules.break_cost(board) <= score
}

pub struct ClassicV1;

impl Ruleset for ClassicV1 {
    fn break_cost(&self, _board: &Board) -> usize {
        1000
    }
    fn break_max(&self, _board: &Board) -> usize {
        3
    }
    fn break_tile_threshold(&self, _board: &Board) -> usize {
        16
    }

    fn game_over(&self, board: &Board) -> bool {
        !board.has_possible_moves()
    }

    fn won(&self, board: &Board) -> bool {
        board.get_all_tiles().iter().any(|t| t.value >= 2048)
    }
}

pub struct ClassicV2;

impl Ruleset for ClassicV2 {
    fn break_max(&self, _board: &Board) -> usize {
        3
    }
    fn break_cost(&self, board: &Board) -> usize {
        match (board.width, board.height) {
            (2, 2) => 100,
            (2, 3) | (3, 2) => 250,
            (3, 3) => 500,
            (3, 4) | (4, 3) => 750,
            (4, 4) => 1000,
            (4, 5) | (5, 4) => 1250,
            (5, 5) => 1500,
            (5, 6) | (6, 5) => 1750,
            (6, 6) => 2000,
            _ => 2500,
        }
    }
    fn break_tile_threshold(&self, _board: &Board) -> usize {
        16
    }

    fn game_over(&self, board: &Board) -> bool {
        !board.has_possible_moves()
    }

    fn won(&self, board: &Board) -> bool {
        board.get_all_tiles().iter().any(|t| t.value >= 2048)
    }
}
