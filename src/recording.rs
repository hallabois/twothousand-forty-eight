//! Provides [Recording], to hold recorded games

use crate::board::Tiles;
use crate::board::{tile::Tile, MoveError};
use crate::direction::Direction;

use serde::{Deserialize, Serialize};

pub type History = Vec<(Tiles, Direction, Option<Tile>)>;

/// Represents a recording of a played game of 2048, usually parsed from a string with [parser](crate::parser).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recording {
    /// The width of the recorded game
    pub width: usize,

    /// The height of the recorded game
    pub height: usize,

    /// The move history, containing data about the current board data at each index along the direction of the move and a possible tile to be added to the board between the moves.
    pub history: History,
}

impl Recording {
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
        format!("{:X}", hasher.finalize())
    }
}

impl std::fmt::Display for Recording {
    /// Converts the recording back to a format the [parser](crate::parser) can read.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = "".to_owned();
        for (index, i) in self.clone().history.into_iter().enumerate() {
            let board = crate::board::Board {
                tiles: i.0,
                width: self.width,
                height: self.height,
                ..Default::default()
            };
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
            out += i.1.get_index();
            if index < self.history.len() - 1 {
                out += ":";
            }
        }
        write!(f, "{}", out)
    }
}

/// Represents a seeded recording of a played game of 2048. Unlike [Recording] a [SeededRecording] can be parsed by just using tryfrom.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeededRecording {
    pub version: u8,
    pub seed: usize,
    pub width: u8,
    pub height: u8,
    pub moves: Vec<Direction>,
}

impl SeededRecording {
    pub fn new(version: u8, seed: usize, width: u8, height: u8, moves: Vec<Direction>) -> Self {
        Self {
            version,
            seed,
            width,
            height,
            moves,
        }
    }
}

use thiserror::Error;
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum SeededRecordingParseError {
    #[error("missing version information")]
    MissingVersion,
    #[error("invalid version information")]
    InvalidVersion,
    #[error("missing seed")]
    MissingSeed,
    #[error("invalid seed")]
    InvalidSeed,
    #[error("missing width")]
    MissingWidth,
    #[error("invalid width")]
    InvalidWidth,
    #[error("missing height")]
    MissingHeight,
    #[error("invalid height")]
    InvalidHeight,
    #[error("missing moves")]
    MissingMoves,
    #[error("invalid move")]
    InvalidMove,
}

/// Converts a string to a [SeededRecording].
/// Schema:
///    [version]:[seed]:[width]:[height]:[moves]
/// where [moves] is a base64 encoded string of the moves, each move is represented by a single byte with 5 possible states:
/// 0: Up
/// 1: Down
/// 2: Left
/// 3: Right
/// 4: None / End / Padding
impl TryFrom<&str> for SeededRecording {
    type Error = SeededRecordingParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(":");
        let version = split
            .next()
            .ok_or(SeededRecordingParseError::MissingVersion)?
            .parse::<u8>()
            .map_err(|_| SeededRecordingParseError::InvalidVersion)?;
        let seed = split
            .next()
            .ok_or(SeededRecordingParseError::MissingSeed)?
            .parse::<usize>()
            .map_err(|_| SeededRecordingParseError::InvalidSeed)?;
        let width = split
            .next()
            .ok_or(SeededRecordingParseError::MissingWidth)?
            .parse::<u8>()
            .map_err(|_| SeededRecordingParseError::InvalidWidth)?;
        let height = split
            .next()
            .ok_or(SeededRecordingParseError::MissingHeight)?
            .parse::<u8>()
            .map_err(|_| SeededRecordingParseError::InvalidHeight)?;
        let moves = split
            .next()
            .ok_or(SeededRecordingParseError::MissingMoves)?;
        let mut base = convert_base::Convert::new(64, 6);
        let coded = base64::decode(moves).map_err(|_| SeededRecordingParseError::InvalidMove)?;
        let moves = base.convert::<u8, u8>(&coded);
        let moves = moves
            .iter()
            .map(|dir| match dir {
                1 => Direction::UP,
                2 => Direction::RIGHT,
                3 => Direction::DOWN,
                4 => Direction::LEFT,
                5 => Direction::BREAK,
                _ => Direction::END,
            })
            .collect::<Vec<Direction>>();

        Ok(Self {
            version,
            seed,
            width,
            height,
            moves,
        })
    }
}

impl From<&SeededRecording> for String {
    fn from(recording: &SeededRecording) -> Self {
        let mut out = String::new();
        out += recording.version.to_string().as_str();
        out += ":";
        out += recording.seed.to_string().as_str();
        out += ":";
        out += recording.width.to_string().as_str();
        out += ":";
        out += recording.height.to_string().as_str();
        out += ":";
        let mut base = convert_base::Convert::new(6, 64);
        let input: Vec<u8> = recording
            .moves
            .iter()
            .map(|dir| match dir {
                Direction::UP => 1,
                Direction::RIGHT => 2,
                Direction::DOWN => 3,
                Direction::LEFT => 4,
                Direction::BREAK => 5,
                Direction::END => 0,
                Direction::START => 0,
            })
            .collect();
        let z = base.convert::<u8, u8>(&input);
        let moves = base64::encode(z);
        out += moves.as_str();
        out
    }
}

impl SeededRecording {
    pub fn get_board_at_move(
        &self,
        move_index: usize,
        id_assignment: crate::board::tile_id_assigner::IDAssignment,
    ) -> Result<crate::board::Board, MoveError> {
        let mut board = crate::board::Board::new(
            self.width as usize,
            self.height as usize,
            id_assignment,
            Some(self.seed),
        );
        for i in 0..=move_index {
            board.move_in_direction(self.moves[i])?;
        }
        Ok(board)
    }
}
