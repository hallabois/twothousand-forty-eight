//! Provides [Direction] to represent move directions

#[cfg(feature = "wasm")]
use serde::{Serialize, Deserialize};

/// A representation of the possible move directions during the game.
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub enum Direction{
    UP,
    RIGHT,
    DOWN,
    LEFT,

    /// The game ended and this placeholder direction was provided
    END
}

/// A list of all the directions excluding the empty placeholder END direction
pub const REAL_DIRECTIONS: [Direction; 4] = [
    Direction::UP,
    Direction::RIGHT,
    Direction::DOWN,
    Direction::LEFT
];
impl Direction {

    /// Get the X component of a direction, e.g. UP => 0, RIGHT => 1
    pub fn get_x(&self) -> i64{
        match self{
            Self::UP => 0,
            Self::RIGHT => 1,
            Self::DOWN => 0,
            Self::LEFT => -1,
            Self::END => 0
        }
    }

    /// Get the Y component of a direction, e.g. UP => 1, RIGHT => 0
    pub fn get_y(&self) -> i64{
        match self{
            Self::UP => -1,
            Self::RIGHT => 0,
            Self::DOWN => 1,
            Self::LEFT => 0,
            Self::END => 0
        }
    }

    /// Get the index of the direction in the order: up, right, down, left
    pub fn get_index(&self) -> &str{
        match self{
            Self::UP => "0",
            Self::RIGHT => "1",
            Self::DOWN => "2",
            Self::LEFT => "3",
            Self::END => "e"
        }
    }

    /// Get the corresponding direction of an index in the order: up, right, down, left
    pub fn from_index(index: usize) -> Direction{
        match index{
            0 => Self::UP,
            1 => Self::RIGHT,
            2 => Self::DOWN,
            3 => Self::LEFT,
            _ => Self::END
        }
    }

    /// Get the corresponding direction of an index in the order: up, right, down, left
    pub fn from_index_str(index: &str) -> Direction{
        match index{
            "0" => Self::UP,
            "1" => Self::RIGHT,
            "2" => Self::DOWN,
            "3" => Self::LEFT,
            _ => Self::END
        }
    }
}