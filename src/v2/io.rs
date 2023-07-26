use std::{num::ParseIntError, str::FromStr};

use base64::Engine;
use thiserror::Error;

use crate::direction::Direction;

use super::recording::SeededRecording;

// do NOT change this, it will break all existing seeded recordings
pub(crate) const SEEDED_RECORDING_SEPARATOR: &str = ":";

#[derive(Error, Debug, Clone)]
pub enum SeededRecordingParseError {
    #[error("unknown format")]
    MissingReservedSpaceStart,
    #[error("unknown format")]
    MissingReservedSpaceEnd,
    #[error("missing version information")]
    MissingVersion,
    #[error("invalid version information: {0}")]
    InvalidVersion(#[source] ParseIntError),
    #[error("unsupported version: {0}")]
    UnsupportedVersion(u8),
    #[error("missing seed")]
    MissingSeed,
    #[error("invalid seed: {0}")]
    InvalidSeed(#[source] ParseIntError),
    #[error("missing width")]
    MissingWidth,
    #[error("invalid width: {0}")]
    InvalidWidth(#[source] ParseIntError),
    #[error("missing height")]
    MissingHeight,
    #[error("invalid height: {0}")]
    InvalidHeight(#[source] ParseIntError),
    #[error("missing moves")]
    MissingMoves,
    #[error("invalid move")]
    InvalidMove,
}

/// Converts a string to a [SeededRecording].
/// Schema:
///    ::(version):(width):(height):(seed):(moves)\n(arbitrary data)
/// where moves is a base64 encoded string of the moves, each move is represented by a single byte with 5 possible states:
/// 0: Up
/// 1: Down
/// 2: Left
/// 3: Right
/// 4: None / End / Padding
impl FromStr for SeededRecording {
    type Err = SeededRecordingParseError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        // We only care about the first line to allow for comments
        let first_line = data.lines().next().unwrap_or(data);
        let mut split = first_line.split(SEEDED_RECORDING_SEPARATOR);
        let _reserved_space_start = split
            .next()
            .ok_or(SeededRecordingParseError::MissingReservedSpaceStart)?;
        let _reserved_space_end = split
            .next()
            .ok_or(SeededRecordingParseError::MissingReservedSpaceEnd)?;
        let version = split
            .next()
            .ok_or(SeededRecordingParseError::MissingVersion)?
            .parse::<u8>()
            .map_err(SeededRecordingParseError::InvalidVersion)?;
        match version {
            2 => parse_v2(
                split
                    .collect::<Vec<_>>()
                    .join(SEEDED_RECORDING_SEPARATOR)
                    .as_str(),
            ),
            _ => Err(SeededRecordingParseError::UnsupportedVersion(version)),
        }
    }
}
fn parse_v2(data: &str) -> Result<SeededRecording, SeededRecordingParseError> {
    let mut split = data.split(SEEDED_RECORDING_SEPARATOR);
    let width = split
        .next()
        .ok_or(SeededRecordingParseError::MissingWidth)?
        .parse::<usize>()
        .map_err(SeededRecordingParseError::InvalidWidth)?;
    let height = split
        .next()
        .ok_or(SeededRecordingParseError::MissingHeight)?
        .parse::<usize>()
        .map_err(SeededRecordingParseError::InvalidHeight)?;
    let seed = split
        .next()
        .ok_or(SeededRecordingParseError::MissingSeed)?
        .parse::<usize>()
        .map_err(SeededRecordingParseError::InvalidSeed)?;
    let moves = split
        .next()
        .ok_or(SeededRecordingParseError::MissingMoves)?;
    let mut base = convert_base::Convert::new(64, 6);
    let b64 = get_b64_engine();
    let coded = b64
        .decode(moves)
        .map_err(|_| SeededRecordingParseError::InvalidMove)?;
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

    Ok(SeededRecording {
        version: 2,
        seed,
        width,
        height,
        moves,
    })
}

impl From<&SeededRecording> for String {
    fn from(recording: &SeededRecording) -> Self {
        let mut out = String::new();
        out += SEEDED_RECORDING_SEPARATOR; // reserved space start
        out += SEEDED_RECORDING_SEPARATOR; // reserved space end
        out += recording.version.to_string().as_str();
        out += SEEDED_RECORDING_SEPARATOR;
        out += recording.width.to_string().as_str();
        out += SEEDED_RECORDING_SEPARATOR;
        out += recording.height.to_string().as_str();
        out += SEEDED_RECORDING_SEPARATOR;
        out += recording.seed.to_string().as_str();
        out += SEEDED_RECORDING_SEPARATOR;
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
        let b64 = get_b64_engine();
        let moves = b64.encode(z);
        out += moves.as_str();
        out
    }
}

fn get_b64_engine() -> base64::engine::general_purpose::GeneralPurpose {
    base64::engine::general_purpose::STANDARD_NO_PAD
}

#[cfg(test)]
mod tests {
    use crate::{
        unified::hash::Hashable,
        v2::{recording::SeededRecording, test_data},
    };

    #[test]
    fn parse() {
        let data = test_data::GAME_EBAY;
        data.parse::<SeededRecording>().unwrap();
    }

    #[test]
    fn comments() {
        let data = test_data::GAME_EBAY_COMMENTED;
        data.parse::<SeededRecording>().unwrap();
    }

    #[test]
    fn comments_hash() {
        let data = test_data::GAME_EBAY_COMMENTED;
        assert_eq!(
            data.parse::<SeededRecording>().unwrap().game_hash(),
            test_data::GAME_EBAY_HASH
        );
    }
}
