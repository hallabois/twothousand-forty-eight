//! Unified interface for parsing and validating all versions of the recording format.
//!
//! Version-specific code is available in the [v1](crate::v1) and [v2](crate::v2) modules, they may be of use if you know the version of the recording you are working with beforehand.

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{
    v1,
    v2::{self, io::SEEDED_RECORDING_SEPARATOR, recording::SeededRecording},
};

use self::{
    hash::Hashable,
    reconstruction::Reconstructable,
    validation::{Validatable, ValidationResult},
};

pub mod game;
pub mod hash;
pub mod reconstruction;
pub mod validation;

const ERR_UNSUPPORTED_VERSION: &str = "unsupported protocol version";

pub fn detect_version(data: &str) -> Option<u8> {
    let modern_prefix: String = format!(
        "{}{}",
        SEEDED_RECORDING_SEPARATOR, SEEDED_RECORDING_SEPARATOR
    );
    if data.starts_with(&modern_prefix) {
        return data.split(&modern_prefix).nth(1).and_then(|version| {
            version
                .split(SEEDED_RECORDING_SEPARATOR)
                .next()?
                .parse()
                .ok()
        });
    }
    Some(1)
}

pub fn validate(data: &str) -> anyhow::Result<ValidationResult> {
    match detect_version(data) {
        Some(1) => {
            let parsed = v1::parser::parse_data(data)?;
            Ok(parsed.validate()?)
        }
        Some(2) => {
            let parsed: v2::recording::SeededRecording = data.parse()?;
            Ok(parsed.validate()?)
        }
        _ => Err(anyhow!(ERR_UNSUPPORTED_VERSION)),
    }
}

pub fn reconstruct(data: &str) -> anyhow::Result<reconstruction::HistoryReconstruction> {
    match detect_version(data) {
        Some(1) => {
            let parsed = v1::parser::parse_data(data)?;
            Ok(parsed.reconstruct()?)
        }
        Some(2) => {
            let parsed: v2::recording::SeededRecording = data.parse()?;
            Ok(parsed.reconstruct()?)
        }
        _ => Err(anyhow!(ERR_UNSUPPORTED_VERSION)),
    }
}

pub fn get_gamestate(data: &str) -> anyhow::Result<game::GameState> {
    match detect_version(data) {
        Some(1) => {
            let parsed = v1::parser::parse_data(data)?;
            Ok(game::GameState::from_reconstructable_ruleset(&parsed)?)
        }
        Some(2) => {
            let parsed: v2::recording::SeededRecording = data.parse()?;
            Ok(game::GameState::from_reconstructable_ruleset(&parsed)?)
        }
        _ => Err(anyhow!(ERR_UNSUPPORTED_VERSION)),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg(feature = "wasm")]
#[derive(tsify::Tsify)]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub enum ParseResult {
    V1(v1::recording::Recording),
    V2(v2::recording::SeededRecording),
}

pub fn parse(data: &str) -> anyhow::Result<ParseResult> {
    match detect_version(data) {
        Some(1) => Ok(ParseResult::V1(v1::parser::parse_data(data)?)),
        Some(2) => Ok(ParseResult::V2(data.parse()?)),
        _ => Err(anyhow!(ERR_UNSUPPORTED_VERSION)),
    }
}

pub fn hash(data: &str) -> anyhow::Result<String> {
    match detect_version(data) {
        Some(1) => Ok(v1::parser::parse_data(data)?.game_hash()),
        Some(2) => Ok(data.parse::<SeededRecording>()?.game_hash()),
        _ => Err(anyhow!(ERR_UNSUPPORTED_VERSION)),
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn detect_v1() {
        let data = "4x4Sfakedatahere";
        assert_eq!(super::detect_version(data), Some(1));
    }
    #[test]
    fn detect_v2() {
        let data = "::2:fakedatahere";
        assert_eq!(super::detect_version(data), Some(2));
    }

    #[test]
    fn validate_v1() {
        let data = crate::v1::tests::lib_testgames::GAME4X4;
        assert!(super::validate(data).is_ok());
    }

    #[test]
    fn validate_v2() {
        let data = crate::v2::test_data::GAME_EBAY;
        assert!(super::validate(data).is_ok());
    }

    #[test]
    fn gamestate_v1() {
        let data = crate::v1::tests::lib_testgames::GAME4X4;
        assert!(super::get_gamestate(data).is_ok());
    }

    #[test]
    fn gamestate_v2() {
        let data = crate::v2::test_data::GAME_EBAY;
        assert!(super::get_gamestate(data).is_ok());
    }
}
