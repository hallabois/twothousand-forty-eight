//! Provides [Board] to hold game board data and [tile] to hold the values of the board tiles.

pub mod tile;
use std::fmt::Display;

use crate::direction::Direction;
use serde::{Deserialize, Serialize};
use tile::Tile;

use self::tile::InitialID;

/// Max width of a board the program can handle. Be careful when increasing, as this increases memory use expotentially.
pub const MAX_WIDTH: usize = 6;
/// Max height of a board the program can handle. Be careful when increasing, as this increases memory use expotentially.
pub const MAX_HEIGHT: usize = 6;

#[cfg_attr(feature = "wasm", tsify::declare)]
pub type Tiles = [[Option<Tile>; MAX_WIDTH]; MAX_HEIGHT];

/// Holds game board data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg(feature = "wasm")]
#[derive(tsify::Tsify)]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Board {
    /// The width of the board. Value of 0 is untested
    pub width: usize,

    /// The height of the board. Value of 0 is untested
    pub height: usize,

    /// The tiles of the board, note that the size of the array is allocated based on the max size.
    pub tiles: Tiles,

    /// State of the random number generator
    pub rng_state: u32,

    /// State of the id generator, the last id that was assigned
    id_counter: usize,
}

impl Board {
    /// Create a new board with a specified width and height and initialize all tiles
    pub fn new(width: usize, height: usize, seed: u32) -> Board {
        let mut id_counter = 0;
        Board {
            width,
            height,
            tiles: initialize_tiles(width, height, &mut id_counter),
            rng_state: seed,
            id_counter,
        }
    }

    /// Set a tile on the board and silently fail if the target tile doesn't exist.
    pub fn set_tile(&mut self, x: usize, y: usize, val: usize) {
        if self.tiles[y][x].is_some() {
            self.tiles[y][x] = Some(Tile::new(
                x,
                y,
                val,
                tile::InitialID::AutoAssign(&mut self.id_counter),
            ));
        }
    }

    /// Get the tiles that exist and which's values are non-zero
    pub fn get_occupied_tiles(&self) -> Vec<Tile> {
        let mut out: Vec<Tile> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(tile) = self.tiles[y][x] {
                    if tile.value != 0 {
                        out.push(tile)
                    }
                }
            }
        }
        out
    }

    /// Get the tiles that exist and which's values are zero
    pub fn get_non_occupied_tiles(&self) -> Vec<Tile> {
        let mut out: Vec<Tile> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(tile) = self.tiles[y][x] {
                    if tile.value == 0 {
                        out.push(tile)
                    }
                }
            }
        }
        out
    }

    /// Get all tiles that exist ( aren't None )
    pub fn get_all_tiles(&self) -> Vec<Tile> {
        let mut out: Vec<Tile> = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(tile) = self.tiles[y][x] {
                    out.push(tile);
                }
            }
        }
        out
    }

    /// Get the combined value of all the tiles
    pub fn get_total_value(&self) -> usize {
        self.get_all_tiles().iter().map(|t| t.value).sum()
    }

    /// Get the sum of all the tile ids, this is used for testing and debugging
    // TODO: REMOVE
    pub fn get_id_sum(&self) -> usize {
        self.get_all_tiles().iter().map(|t| t.id).sum()
    }

    /// Check if a move in any direction is possible
    pub fn has_possible_moves(&self) -> bool {
        // If there are any empty tiles, there are possible moves
        if !self.get_non_occupied_tiles().is_empty() {
            return true;
        }

        // Check if any tiles can merge with their neighbours instead
        const NEIGHBOUR_DIRECTIONS: [Direction; 4] = [
            Direction::UP,
            Direction::RIGHT,
            Direction::DOWN,
            Direction::LEFT,
        ];
        for t in self.get_occupied_tiles() {
            for dir in NEIGHBOUR_DIRECTIONS {
                let (off_x, off_y) = (dir.get_x(), dir.get_y());
                let x: i64 = t.x as i64 + off_x;
                let y: i64 = t.y as i64 + off_y;
                if (x < 0 || y < 0) || (x as usize > self.width - 1 || y as usize > self.height - 1)
                {
                    // Out of bounds
                    continue;
                }
                if let Some(neighbour) = self.tiles[y as usize][x as usize] {
                    if t.value == neighbour.value {
                        // There is a possible merge
                        return true;
                    }
                }
            }
        }

        // No possible moves
        false
    }

    pub fn get_random_tile_to_add(&mut self) -> Option<tile::Tile> {
        use crate::random::Pickable;

        let possible = self.get_non_occupied_tiles();
        if !possible.is_empty() {
            let t = possible.pick_lcg(&mut self.rng_state);

            let value = tile::Tile::random_value(&mut self.rng_state);

            return Some(tile::Tile::new(t.x, t.y, value, tile::InitialID::Id(t.id)));
        }
        None
    }

    pub fn add_random_tile(&mut self) {
        let possible_t = self.get_random_tile_to_add();
        match possible_t {
            None => {}
            Some(t) => {
                self.set_tile(t.x, t.y, t.value);
            }
        }
    }

    /// Move the board in the direction "dir" and return the score gained from the move
    pub fn move_in_direction(&mut self, dir: Direction) -> Result<usize, MoveError> {
        let result = check_move(*self, dir);
        result.map(|data| {
            *self = data.board;
            data.score_gain
        })
    }
}

/// Initialize a new 4x4 board with [Board::new] and a random seed
impl Default for Board {
    fn default() -> Board {
        Board::new(4, 4, rand::random::<u32>())
    }
}

impl From<(Tiles, u32)> for Board {
    fn from((tiles, rng_state): (Tiles, u32)) -> Self {
        let height = tiles.len();
        let width = if height > 0 { tiles[0].len() } else { 0 };
        let largest_id = tiles
            .iter()
            .flatten()
            .filter_map(|t| t.as_ref())
            .map(|t| t.id)
            .max()
            .unwrap_or(0);
        Board {
            width,
            height,
            tiles,
            rng_state,
            id_counter: largest_id + 1,
        }
    }
}

/// Return a debug visualization of the board
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                match self.tiles[y][x] {
                    Some(i) => {
                        let string = i.value.to_string();
                        out += &format!("{}\t", string.as_str());
                    }
                    None => {
                        out += "?\t";
                    }
                }
            }
            out += "\n";
        }
        write!(f, "{}", out)
    }
}

/// Initialize an array of empty tiles created with [Tile::new]
pub fn initialize_tiles(width: usize, height: usize, id_counter: &mut usize) -> Tiles {
    if width > MAX_WIDTH || height > MAX_HEIGHT {
        panic!("Board size too big! This version of the program has been compiled to support the maximum size of {:?}", (MAX_WIDTH, MAX_HEIGHT));
    }
    let mut tiles: Tiles = [[None; MAX_WIDTH]; MAX_HEIGHT];
    for x in 0..width {
        for y in 0..height {
            tiles[y][x] = Some(Tile::new(x, y, 0, tile::InitialID::AutoAssign(id_counter)));
        }
    }
    tiles
}

/// Return the closest tile with the value of "mask" to the tile "t" in the given direction "dir",
/// if None is returned, no such tile was found.
pub fn get_closest_tile(
    t: Tile,
    viable_tiles: &Vec<Tile>,
    dir: Direction,
    mask: Option<usize>,
) -> Option<Tile> {
    let dir_x = dir.get_x();
    let dir_y = dir.get_y();
    let move_is_vertical = dir_y == 0;
    let vel = if move_is_vertical { dir_x } else { dir_y };

    let mut closest = None;
    let mut closest_dist: usize = usize::MAX;
    let mut nearest_blocking = usize::MAX;

    let (a1, a2) = if move_is_vertical {
        (t.x, t.y)
    } else {
        (t.y, t.x)
    };
    for i in viable_tiles {
        let (b1, b2) = if move_is_vertical {
            (i.x, i.y)
        } else {
            (i.y, i.x)
        };
        let correct_direction = if vel > 0 { a1 < b1 } else { a1 > b1 };
        let same_axis = a2 == b2;

        if same_axis && correct_direction {
            let distance = if vel > 0 { b1 - a1 } else { a1 - b1 };

            if distance != 0 && distance < closest_dist {
                let recursed = get_closest_tile(*i, viable_tiles, dir, mask);
                if let Some(r) = recursed {
                    let mask_matches = match mask {
                        Some(m) => r.value == m,
                        None => true,
                    };
                    if mask_matches && r.merged_from.is_none() {
                        // Let this tile merge with the one in the direction of the move
                        nearest_blocking = distance;
                    } else {
                        closest = Some(*i);
                        closest_dist = distance;
                    }
                } else {
                    closest = Some(*i);
                    closest_dist = distance;
                }
            }
        }
    }
    if nearest_blocking < closest_dist {
        return None;
    }
    closest
}

/// Return the farthest tile with the value of "mask" to the tile "t" in the given direction "dir",
/// if None is returned, no such tile was found.
pub fn get_farthest_tile(
    t: Tile,
    all_tiles: &Vec<Tile>,
    dir: Direction,
    mask: Option<usize>,
) -> Option<Tile> {
    let dir_x = dir.get_x();
    let dir_y = dir.get_y();
    let move_is_vertical = dir_y == 0;
    let vel = if move_is_vertical { dir_x } else { dir_y };

    let mut farthest = None;
    let mut farthest_dist: usize = usize::MIN;
    let mut nearest_blocking = usize::MAX;

    let (a1, a2) = if move_is_vertical {
        (t.x, t.y)
    } else {
        (t.y, t.x)
    };
    for i in all_tiles {
        let (b1, b2) = if move_is_vertical {
            (i.x, i.y)
        } else {
            (i.y, i.x)
        };
        let correct_direction = if vel > 0 { a1 < b1 } else { a1 > b1 };
        let same_axis = a2 == b2;
        if same_axis && correct_direction {
            let distance = if vel > 0 { b1 - a1 } else { a1 - b1 };
            let mask_matches = match mask {
                Some(m) => i.value == m,
                None => true,
            };

            if distance != 0 && distance > farthest_dist && mask_matches {
                farthest = Some(*i);
                farthest_dist = distance;
            } else if distance != 0 && !mask_matches && distance < nearest_blocking {
                nearest_blocking = distance;
            }
        }
    }
    if nearest_blocking < farthest_dist {
        return None;
    }
    farthest
}

const MAX_MOVE_CHECKS: usize = 256;
#[derive(Serialize, Deserialize)]
pub struct MoveResult {
    pub board: Board,
    pub score_gain: usize,
}

use thiserror::Error;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Error)]
pub enum MoveError {
    #[error("No valid moves left")]
    NoValidMovesLeft,
    #[error("Move has no effect")]
    HasNoEffect,
    #[error("Break not allowed")]
    BreakNotAllowed,
}

/// Check if a move is possible in the direction "dir"
pub fn check_move(board: Board, dir: Direction) -> Result<MoveResult, MoveError> {
    // Copy the board so we don't modify the original, necessary for preserving the random state
    let mut board = board;
    if dir == Direction::END {
        return Ok(MoveResult {
            board,
            score_gain: 0,
        });
    }

    if !board.has_possible_moves() {
        return Err(MoveError::NoValidMovesLeft);
    }

    let mut was_changed = false;

    // Copy the current board and unset merged_from and new
    for t in board.get_occupied_tiles() {
        board.tiles[t.y][t.x] = Some(Tile {
            merged_from: None,
            new: false,
            ..t
        });
    }

    let mut score = 0;

    // Merge
    let mut ids_checked_for_merge: Vec<usize> = vec![];
    for _ in 0..MAX_MOVE_CHECKS {
        let occupied_tiles = board.get_occupied_tiles();
        let viable_tiles: Vec<Tile> = occupied_tiles
            .iter()
            .filter(|t| t.merged_from.is_none())
            .copied()
            .collect();
        if let Some(t) = viable_tiles
            .iter()
            .find(|t| !ids_checked_for_merge.contains(&t.id))
        {
            if let Some(closest) = get_closest_tile(*t, &occupied_tiles, dir, Some(t.value)) {
                if t.value == closest.value && closest.merged_from.is_none() {
                    board.tiles[t.y][t.x] = Some(Tile::new(
                        t.x,
                        t.y,
                        0,
                        InitialID::AutoAssign(&mut board.id_counter),
                    ));

                    let mut merged = Tile::new(
                        closest.x,
                        closest.y,
                        closest.value * 2,
                        InitialID::AutoAssign(&mut board.id_counter),
                    );
                    merged.merged_from = Some([t.id, closest.id]);

                    score += merged.value;
                    board.tiles[closest.y][closest.x] = Some(merged);
                    was_changed = true;
                }
            }
            ids_checked_for_merge.push(t.id);
        } else {
            break;
        }
    }

    // Slide
    let mut moved_tiles: Vec<usize> = vec![];
    for _ in 0..MAX_MOVE_CHECKS {
        let tiles_post = board.get_occupied_tiles();

        if let Some(t) = tiles_post.iter().find(|t| !moved_tiles.contains(&t.id)) {
            let all_tiles = board.get_all_tiles();
            let dir_to_use = dir;

            if let Some(farthest_free) = get_farthest_tile(*t, &all_tiles, dir_to_use, Some(0)) {
                let new_tile: Tile = Tile {
                    x: farthest_free.x,
                    y: farthest_free.y,
                    ..*t
                };

                board.tiles[t.y][t.x] = Some(Tile::new(
                    t.x,
                    t.y,
                    0,
                    tile::InitialID::AutoAssign(&mut board.id_counter),
                ));
                board.tiles[farthest_free.y][farthest_free.x] = Some(new_tile);

                was_changed = true;
                moved_tiles = vec![];
            } else {
                moved_tiles.push(t.id);
            }
        } else {
            break;
        }
    }

    if !was_changed {
        return Err(MoveError::HasNoEffect);
    }
    Ok(MoveResult {
        board,
        score_gain: score,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{direction, random::Pickable};

    use super::*;
    fn ensure_no_same_ids(board: &Board) {
        let mut seen_ids: HashSet<usize> = HashSet::new();
        for t in board.get_all_tiles() {
            if seen_ids.contains(&t.id) {
                panic!("Already seen ID: {}", t.id);
            } else {
                seen_ids.insert(t.id);
            }
        }
    }

    #[test]
    #[should_panic]
    fn equal_id_test_works() {
        let mut board = Board::default();
        board.tiles[0][0] = Some(Tile::new(0, 0, 2, InitialID::Id(123123)));
        board.tiles[1][1] = Some(Tile::new(1, 1, 4, InitialID::Id(123123)));
        ensure_no_same_ids(&board);
    }

    #[test]
    fn no_equal_ids_on_init() {
        let board = Board::default();
        ensure_no_same_ids(&board);
    }

    #[test]
    fn no_equal_ids_on_play() {
        let mut board = Board::default();
        board.add_random_tile();
        board.add_random_tile();
        ensure_no_same_ids(&board);
        for i in 0..500 {
            let mut fake_seed = i;
            let dir = direction::MOVE_DIRECTIONS.pick_lcg(&mut fake_seed);
            if board.move_in_direction(*dir).is_ok() {
                board.add_random_tile();
            }
            ensure_no_same_ids(&board);
        }
    }
}
