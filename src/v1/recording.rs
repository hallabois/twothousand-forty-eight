//! Provides [Recording], to hold recorded games

use crate::board::{tile::Tile, Tiles};
use crate::direction::Direction;
use crate::unified::hash::Hashable;

use serde::{Deserialize, Serialize};

pub type History = Vec<(Tiles, Direction, Option<Tile>)>;

/// Represents a recording of a played game of 2048, usually parsed from a string with [parser](crate::v1::parser).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub struct Recording {
    /// The width of the recorded game
    pub width: usize,

    /// The height of the recorded game
    pub height: usize,

    /// The move history, containing data about the current board data at each index along the direction of the move and a possible tile to be added to the board between the moves.
    pub history: History,
}

impl Hashable for Recording {
    /// Returns a hash of the history. The hash is only composed from the move directions and is not affected by any board data changes.
    /// This is far cheaper and invalid board data or score changes should be catched by the [validator](crate::v1::validator) anyways.
    fn game_hash(&self) -> String {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(self.width.to_string().as_bytes());
        hasher.update(self.height.to_string().as_bytes());
        for i in &self.history {
            hasher.update(i.1.get_shorthand().as_bytes());
        }
        format!("{:X}", hasher.finalize())
    }
}

impl std::fmt::Display for Recording {
    /// Converts the recording back to a format the [parser](crate::v1::parser) can read.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = "".to_owned();
        for (index, i) in self.clone().history.into_iter().enumerate() {
            let board = crate::board::Board::from((i.0, 0));
            let tiles = board.get_all_tiles();
            out += tiles
                .iter()
                .map(|t| t.value.to_string())
                .collect::<Vec<String>>()
                .join(".")
                .as_str();
            out += "+";
            if let Some(t) = i.2 {
                out = out
                    + t.x.to_string().as_str()
                    + ","
                    + t.y.to_string().as_str()
                    + "."
                    + t.value.to_string().as_str()
            }
            out += ";";
            out += i.1.get_shorthand();
            if index < self.history.len() - 1 {
                out += ":";
            }
        }
        write!(f, "{}", out)
    }
}
