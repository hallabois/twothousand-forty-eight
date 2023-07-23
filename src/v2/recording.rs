use serde::{Deserialize, Serialize};

use super::game::{replay_moves, MoveReplayError};
use crate::{
    board::MoveError,
    direction::Direction,
    rules::{ClassicV1, ClassicV2, Ruleset},
    v1::validator::{HistoryReconstruction, ValidationData},
};

/// Represents a seeded recording of a played game of 2048.
/// Unlike [crate::v1::recording::Recording] a [SeededRecording] can be parsed by just using `TryFrom`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub struct SeededRecording {
    #[serde(alias = "v")]
    pub version: u8,
    #[serde(alias = "s")]
    pub seed: usize,
    #[serde(alias = "w")]
    pub width: usize,
    #[serde(alias = "h")]
    pub height: usize,
    #[serde(alias = "m")]
    pub moves: Vec<Direction>,
}

#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum BoardFetchError {
    #[error("move index ({0}) out of bounds ({1})")]
    MoveIndexOutOfBounds(usize, usize),
    #[error("invalid move on index {1}: {0:?}")]
    MoveError(MoveError, usize),
}

const SEEDED_RECORDING_CURRENT_VERSION: u8 = 2;
impl SeededRecording {
    pub fn new(seed: usize, width: usize, height: usize, moves: Vec<Direction>) -> Self {
        Self {
            version: SEEDED_RECORDING_CURRENT_VERSION,
            seed,
            width,
            height,
            moves,
        }
    }
    pub fn empty(seed: usize, width: usize, height: usize) -> Self {
        Self::new(seed, width, height, vec![])
    }

    pub fn reconstruct(&self) -> Result<HistoryReconstruction, MoveReplayError> {
        replay_moves(self, &*self.get_ruleset())
    }

    pub fn get_current_board(&self) -> Result<crate::board::Board, MoveReplayError> {
        let reconstruction = self.reconstruct()?;
        // We can unwrap here, replay_moves should always return a valid board
        Ok(*reconstruction.history.last().unwrap())
    }

    pub fn validate(&self) -> Result<ValidationData, MoveReplayError> {
        let reconstruction = self.reconstruct()?;
        Ok(reconstruction.validation_data)
    }

    pub fn get_ruleset(&self) -> Box<dyn Ruleset> {
        match self.version {
            1 => Box::new(ClassicV1),
            2 => Box::new(ClassicV2),
            _ => Box::new(ClassicV2), // we should probably panic here
        }
    }
}
