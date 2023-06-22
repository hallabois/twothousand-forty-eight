use crate::board;
use crate::board::initialize_tiles;
use crate::board::tile_id_assigner::IDAssignment;
use crate::direction::Direction;
use crate::random::RandAlgo;
use crate::recording::History;
use crate::recording::Recording;
use crate::validator::MAX_ALLOWED_BREAKS;

use serde::Deserialize;
use serde::Serialize;

use thiserror::Error;
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ParseError {
    #[error("missing width")]
    MissingWidth,
    #[error("missing height")]
    MissingHeight,
    #[error("invalid width")]
    InvalidWidth,
    #[error("invalid height")]
    InvalidHeight,

    #[error("missing history")]
    MissingHistory,

    #[error("missing board data on move {0}")]
    MissingBoard(usize),
    #[error("missing move direction on move {0}")]
    MissingDirection(usize),

    #[error("missing tile value on move {0}")]
    InvalidTileValue(usize),

    #[error("missing x position for addition on move {0}")]
    MissingAddX(usize),
    #[error("missing y position for addition on move {0}")]
    MissingAddY(usize),
    #[error("missing value for addition on move {0}")]
    MissingAddValue(usize),
    #[error("invalid x position for addition on move {0}")]
    InvalidAddX(usize),
    #[error("invalid y position for addition on move {0}")]
    InvalidAddY(usize),
    #[error("invalid value for addition on move {0}")]
    InvalidAddValue(usize),
    #[error("invalid addition on move {0}")]
    InvalidAddition(usize),
}

/// Parses a string representation of a played 2048 game into a [Recording]
///
/// The string should be in the following format:
/// - The string can start with ```[w]x[h]S```, specifying the size of the game board, otherwise it defaults to 4 by 4.
/// (w = width, h = height, both are a [usize])
/// - History indicies should be separated by a single ```:```
///     - ```;``` separates the history index to:
///         - The board data of the move on the left side, separated by a ```+```:
///             - Each tile value of the board, separated from eachother by dots (```.```) on the left side
///             - The tile added to the board after this move on the right side
///                 - The tile's value is on the right side of a comma (```,```) and it's position is on the left side of the comma, separated by a dot (```.```), x first
///         - [Direction] index of the move on the right side
///
/// e.g. ```4x4S0.0.0.0.0.0.0.0.0.0.0.0.0.0.2.2+2,1.2;1```
pub fn parse_data(data: &str) -> Result<Recording, ParseError> {
    let mut history: History = vec![];
    let mut width = 4;
    let mut height = 4;
    let mut historypart = data;
    let parts: Vec<&str> = data.split('S').collect();
    let mut rng_state = 0;
    if parts.len() > 1 {
        historypart = parts.get(1).ok_or(ParseError::MissingHistory)?;
        let dimensions = parts[0].split('x').collect::<Vec<&str>>();
        width = dimensions
            .first()
            .ok_or(ParseError::MissingWidth)?
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidWidth)?;
        height = dimensions
            .get(1)
            .ok_or(ParseError::MissingHeight)?
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidWidth)?;
    }
    for (history_index, step) in historypart.split(':').enumerate() {
        let parts = step.split(';').collect::<Vec<&str>>();
        let bdata = parts[0].split('+').collect::<Vec<&str>>();
        let mut added = "";
        if bdata.len() > 1 {
            added = bdata[1];
        }
        let b = *bdata
            .first()
            .ok_or(ParseError::MissingBoard(history_index))?;
        let mut tiles = initialize_tiles(width, height, IDAssignment::default(), &mut 0);
        let dir = parts
            .get(1)
            .ok_or(ParseError::MissingDirection(history_index))?;
        let direction = Direction::from_index_str(dir);
        for (index, value) in b.split('.').enumerate() {
            let val = value
                .parse::<usize>()
                .map_err(|_| ParseError::InvalidTileValue(history_index))?;
            let x = index % width;
            let y = index / height;
            tiles[y][x] = Some(board::tile::Tile::new(
                x,
                y,
                val,
                board::tile::InitialID::Strategy(IDAssignment::default(), &mut rng_state),
            ));
        }

        let mut added_tile = None;
        if !added.is_empty() {
            let added_vals = added.split('.').collect::<Vec<&str>>();
            let added_index = added_vals
                .first()
                .ok_or(ParseError::InvalidAddition(history_index))?;
            let added_pos = added_index.split(',').collect::<Vec<&str>>();
            let added_x = added_pos
                .first()
                .ok_or(ParseError::MissingAddX(history_index))?
                .parse::<usize>()
                .map_err(|_| ParseError::InvalidAddX(history_index))?;
            let added_y = added_pos
                .get(1)
                .ok_or(ParseError::MissingAddX(history_index))?
                .parse::<usize>()
                .map_err(|_| ParseError::InvalidAddY(history_index))?;
            let added_value = added_vals
                .get(1)
                .ok_or(ParseError::InvalidAddition(history_index))?
                .parse::<usize>()
                .map_err(|_| ParseError::InvalidAddValue(history_index))?;
            added_tile = Some(board::tile::Tile::new(
                added_x,
                added_y,
                added_value,
                board::tile::InitialID::Strategy(IDAssignment::default(), &mut rng_state),
            ));
        }

        history.push((tiles, direction, added_tile));
    }
    Ok(Recording {
        history,
        width,
        height,
    })
}

fn default_size() -> usize {
    4
}

fn default_rng() -> RandAlgo {
    RandAlgo::LCG
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseDataV2 {
    #[serde(alias = "s")]
    pub seed: usize,
    #[serde(alias = "r", default = "default_rng")]
    pub rand: RandAlgo,
    #[serde(alias = "w", default = "default_size")]
    pub width: usize,
    #[serde(alias = "h", default = "default_size")]
    pub height: usize,
    #[serde(alias = "m")]
    pub moves: Vec<Direction>,
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ParseErrorV2 {
    #[error("the string should start with the character \"|\"")]
    MissingMagicMarker,
    #[error("missing seed")]
    MissingSeed,
    #[error("missing width")]
    MissingWidth,
    #[error("missing height")]
    MissingHeight,
    #[error("missing moves")]
    MissingMoves,

    #[error("invalid seed")]
    InvalidSeed,
    #[error("invalid width")]
    InvalidWidth,
    #[error("invalid height")]
    InvalidHeight,

    #[error("unsupported width: {0}")]
    UnsupportedWidth(usize),
    #[error("unsupported height: {0}")]
    UnsupportedHeight(usize),
    #[error("invalid seed: {0}")]
    UnfitSeed(usize),
    #[error("run contains too many breaks: {0} found, {1} allowed")]
    TooManyBreaks(usize, usize),
}

pub fn parse_v2(raw: String) -> Result<ParseDataV2, ParseErrorV2> {
    let split_char = "|";
    let mut split = raw.split(split_char);
    if split.next().is_none() {
        return Err(ParseErrorV2::MissingMagicMarker);
    }
    if split.next().is_none() {
        return Err(ParseErrorV2::MissingMagicMarker);
    }
    let seed = split
        .next()
        .ok_or(ParseErrorV2::MissingSeed)?
        .parse::<usize>()
        .map_err(|_| ParseErrorV2::InvalidSeed)?;
    let width = split
        .next()
        .ok_or(ParseErrorV2::MissingWidth)?
        .parse::<usize>()
        .map_err(|_| ParseErrorV2::InvalidWidth)?;
    let height = split
        .next()
        .ok_or(ParseErrorV2::MissingHeight)?
        .parse::<usize>()
        .map_err(|_| ParseErrorV2::InvalidHeight)?;
    let moves_str = split.next().ok_or(ParseErrorV2::MissingMoves)?;
    let mut moves = vec![];
    for m_c in moves_str.chars() {
        let m = m_c.to_string();
        let dir = Direction::from_index_str(&m);
        moves.push(dir);
    }

    let data = ParseDataV2 {
        seed,
        width,
        height,
        moves,
        rand: default_rng(),
    };

    if data.seed == 0 {
        return Err(ParseErrorV2::UnfitSeed(data.seed));
    }
    if data.width > board::MAX_WIDTH {
        return Err(ParseErrorV2::UnsupportedWidth(data.width));
    }
    if data.height > board::MAX_HEIGHT {
        return Err(ParseErrorV2::UnsupportedHeight(data.height));
    }

    let break_count = data
        .moves
        .iter()
        .filter(|m| **m == Direction::BREAK)
        .count();
    if break_count > MAX_ALLOWED_BREAKS {
        return Err(ParseErrorV2::TooManyBreaks(break_count, MAX_ALLOWED_BREAKS));
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_v2() {
        let d = super::parse_v2(String::from(r#"||12|4|4|00100"#)).unwrap();
        println!("{:?}", d);
    }

    #[test]
    #[should_panic]
    fn parse_v2_invalid_seed() {
        super::parse_v2(String::from(r#"||0|4|4|00100"#)).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_v2_invalid_width() {
        super::parse_v2(String::from(r#"||12|12000220|4|00100"#)).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_v2_invalid_height() {
        super::parse_v2(String::from(r#"||12|4|999838|00100"#)).unwrap();
    }

    #[test]
    #[should_panic]
    fn parse_v2_toomanybreaks() {
        super::parse_v2(String::from(r#"||13|4|4|001b12b033b399b"#)).unwrap();
    }
}
