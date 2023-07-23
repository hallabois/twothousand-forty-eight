//! Provides [Direction] to represent move directions

use serde::{Deserialize, Serialize};

/// A representation of the possible move directions during the game.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Direction {
    /// The game was just started and no move was made on the first frame. Not possible in a real game but some functions need this.
    #[serde(alias = "5", alias = "s")]
    START,

    #[serde(alias = "0")]
    UP,
    #[serde(alias = "1")]
    RIGHT,
    #[serde(alias = "2")]
    DOWN,
    #[serde(alias = "3")]
    LEFT,
    #[serde(alias = "6", alias = "b")]
    BREAK,

    /// The game ended and this placeholder direction was provided
    #[serde(alias = "4", alias = "f")]
    END,
}

/// A list of all the directions excluding the END and START directions
pub const REAL_DIRECTIONS: [Direction; 5] = [
    Direction::UP,
    Direction::RIGHT,
    Direction::DOWN,
    Direction::LEFT,
    Direction::BREAK,
];
/// A list of all the directions a tile can move to
pub const MOVE_DIRECTIONS: [Direction; 4] = [
    Direction::UP,
    Direction::RIGHT,
    Direction::DOWN,
    Direction::LEFT,
];
impl Direction {
    /// Get the X component of a direction, e.g. UP => 0, RIGHT => 1
    pub fn get_x(&self) -> i64 {
        match self {
            Self::UP => 0,
            Self::RIGHT => 1,
            Self::DOWN => 0,
            Self::LEFT => -1,
            Self::END => 0,
            Self::START => 0,
            Self::BREAK => 0,
        }
    }

    /// Get the Y component of a direction, e.g. UP => 1, RIGHT => 0
    pub fn get_y(&self) -> i64 {
        match self {
            Self::UP => -1,
            Self::RIGHT => 0,
            Self::DOWN => 1,
            Self::LEFT => 0,
            Self::END => 0,
            Self::START => 0,
            Self::BREAK => 0,
        }
    }

    /// Get the index of the direction in the order: up, right, down, left
    pub fn get_index(&self) -> &str {
        match self {
            Self::UP => "0",
            Self::RIGHT => "1",
            Self::DOWN => "2",
            Self::LEFT => "3",
            Self::END => "e",
            Self::START => "s",
            Self::BREAK => "b",
        }
    }

    /// Get the corresponding direction of an index in the order: up, right, down, left
    pub fn from_index(index: usize) -> Direction {
        match index {
            0 => Self::UP,
            1 => Self::RIGHT,
            2 => Self::DOWN,
            3 => Self::LEFT,
            6 => Self::BREAK,
            _ => Self::END,
        }
    }

    /// Get the corresponding direction of an index in the order: up, right, down, left
    pub fn from_index_str(index: &str) -> Direction {
        match index {
            "0" => Self::UP,
            "1" => Self::RIGHT,
            "2" => Self::DOWN,
            "3" => Self::LEFT,
            "6" => Self::BREAK,
            "b" => Self::BREAK,
            _ => Self::END,
        }
    }
}
