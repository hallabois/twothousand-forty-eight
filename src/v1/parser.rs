use crate::board;
use crate::board::initialize_tiles;
use crate::direction::Direction;
use crate::v1::recording::History;
use crate::v1::recording::Recording;

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
    let mut id_counter = 0;
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
        let mut tiles = initialize_tiles(width, height, &mut id_counter);
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
                board::tile::InitialID::AutoAssign(&mut id_counter),
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
                board::tile::InitialID::AutoAssign(&mut id_counter),
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
