//! Provides [Recording], to hold recorded games

use crate::board::tile::Tile;
use crate::board::Tiles;
use crate::direction::Direction;

#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};

pub type History = Vec<(Tiles, Direction, Option<Tile>)>;

/// Represents a recording of a played game of 2048, usually parsed from a string with [parser](crate::parser).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
pub struct Recording {
    /// The width of the recorded game
    pub width: usize,

    /// The height of the recorded game
    pub height: usize,

    /// The move history, containing data about the current board data at each index along the direction of the move and a possible tile to be added to the board between the moves.
    pub history: History,
}

impl Recording {
    /// Converts the recording back to a format the [parser](crate::parser) can read.
    pub fn to_string(&self) -> String {
        let mut out = "".to_owned();
        let mut index: usize = 0;
        for i in self.clone().history {
            let board = crate::board::Board {
                tiles: i.0,
                width: self.width,
                height: self.height,
            };
            let tiles = board.get_all_tiles();
            out = out
                + tiles
                    .iter()
                    .map(|t| t.value.to_string())
                    .collect::<Vec<String>>()
                    .join(".")
                    .as_str();
            out = out + "+";
            match i.2 {
                None => out = out + "",
                Some(t) => {
                    out = out
                        + t.x.to_string().as_str()
                        + ","
                        + t.y.to_string().as_str()
                        + "."
                        + t.value.to_string().as_str()
                }
            }
            out = out + ";";
            out = out + i.1.get_index();
            if index < self.history.len() - 1 {
                out = out + ":";
            }
            index += 1;
        }
        return out;
    }

    /// Returns a hash of the history. The hash is only composed from the move directions and is not affected by any board data changes.
    /// This is far cheaper and invalid board data or score changes should be catched by the [validator](crate::validator) anyways.
    #[cfg(feature = "history_hash")]
    pub fn hash_v1(&self) -> String {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(self.width.to_string().as_bytes());
        hasher.update(self.height.to_string().as_bytes());
        for i in &self.history {
            hasher.update(i.1.get_index().as_bytes());
        }
        let out = format!("{:X}", hasher.finalize());
        return out;
    }
}
