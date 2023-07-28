use serde::{Deserialize, Serialize};

use super::replay::MoveReplayError;
use crate::{
    board::MoveError,
    direction::Direction,
    rules::{ClassicV1, ClassicV2, Ruleset, RulesetProvider},
    unified::{
        hash::Hashable,
        reconstruction::Reconstructable,
        validation::{Validatable, ValidationResult},
    },
};

/// Represents a seeded recording of a played game of 2048.
///
/// Unlike [crate::v1::recording::Recording], [SeededRecording] can be parsed by just using `TryFrom`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq)]
#[cfg(feature = "wasm")]
#[derive(tsify::Tsify)]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct SeededRecording {
    #[serde(alias = "v")]
    pub version: u8,
    #[serde(alias = "s")]
    pub seed: u32,
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
    pub fn new(seed: u32, width: usize, height: usize, moves: Vec<Direction>) -> Self {
        Self {
            version: SEEDED_RECORDING_CURRENT_VERSION,
            seed,
            width,
            height,
            moves,
        }
    }
    pub fn empty(seed: u32, width: usize, height: usize) -> Self {
        Self::new(seed, width, height, vec![])
    }

    pub fn get_current_board(&self) -> Result<crate::board::Board, MoveReplayError> {
        let reconstruction = self.reconstruct()?;
        // We can unwrap here, replay_moves should always return a valid board
        Ok(*reconstruction.history.last().unwrap())
    }
}

impl Validatable for SeededRecording {
    type Error = MoveReplayError;
    fn validate(&self) -> Result<ValidationResult, Self::Error> {
        let reconstruction = self.reconstruct()?;
        Ok(reconstruction.validation_data)
    }
}

impl RulesetProvider for SeededRecording {
    fn rules(&self) -> &dyn Ruleset {
        match self.version {
            1 => &ClassicV1,
            2 => &ClassicV2,
            _ => &ClassicV2, // we should probably panic here
        }
    }
}

impl Hashable for SeededRecording {
    fn game_hash(&self) -> String {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(self.version.to_string().as_bytes());
        hasher.update(self.seed.to_string().as_bytes());
        hasher.update(self.width.to_string().as_bytes());
        hasher.update(self.height.to_string().as_bytes());
        for i in &self.moves {
            hasher.update(i.get_index().to_string().as_bytes());
        }
        format!("V2{:X}", hasher.finalize())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        unified::hash::Hashable,
        v2::test_data::{
            GAME_NI4FIRM, GAME_NI4FIRM_HASH, GAME_WON_3_BREAKS, GAME_WON_3_BREAKS_HASH,
        },
    };

    use super::SeededRecording;

    #[test]
    fn hash_a() {
        let parsed: SeededRecording = GAME_NI4FIRM.parse().unwrap();
        assert_eq!(parsed.game_hash(), GAME_NI4FIRM_HASH);
    }

    #[test]
    fn hash_b() {
        let parsed: SeededRecording = GAME_WON_3_BREAKS.parse().unwrap();
        assert_eq!(parsed.game_hash(), GAME_WON_3_BREAKS_HASH);
    }
}
