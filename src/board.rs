//! Provides [Board] to hold game board data and [tile] to hold the values of the board tiles.

pub mod tile;
use crate::direction::Direction;
use serde::{Deserialize, Serialize};
use tile::Tile;

/// Max width of a board the program can handle. Be careful when increasing, as this increases memory use expotentially.
pub const MAX_WIDTH: usize = 5;
/// Max height of a board the program can handle. Be careful when increasing, as this increases memory use expotentially.
pub const MAX_HEIGHT: usize = 5;

pub type Tiles = [[Option<Tile>; MAX_WIDTH]; MAX_HEIGHT];

/// Holds game board data
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Board {
    /// The width of the board. Value of 0 is untested
    pub width: usize,

    /// The height of the board. Value of 0 is untested
    pub height: usize,

    /// The tiles of the board, note that the size of the array is allocated based on the max size.
    pub tiles: Tiles,
}

impl Board {
    /// Create a new board with a [width] and [height] and initialize all tiles
    pub fn new(width: usize, height: usize) -> Board {
        Board {
            width,
            height,
            tiles: create_tiles(width, height),
        }
    }

    /// Set a tile on the board and silently fail if the target tile doesn't exist and [DEBUG_INFO](crate::DEBUG_INFO) is disabled.
    pub fn set_tile(&mut self, x: usize, y: usize, val: usize) {
        if let Some(_) = self.tiles[y][x] {
            self.tiles[y][x] = Some(Tile::new(x, y, val, None));
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
        let mut sum: usize = 0;
        for row in self.tiles {
            for i in row {
                match i {
                    Some(t) => sum += t.value,
                    None => (),
                }
            }
        }
        return sum;
    }
}

/// Initialize a new 4x4 board with [Board::new]
impl Default for Board {
    fn default() -> Board {
        Board::new(4, 4)
    }
}

/// Print a debug visualization of the board
pub fn print_board(
    tiles: [[Option<tile::Tile>; MAX_WIDTH]; MAX_HEIGHT],
    width: usize,
    height: usize,
) {
    println!("{}", board_to_string(tiles, width, height));
}

/// Return a debug visualization of the board
pub fn board_to_string(
    tiles: [[Option<tile::Tile>; MAX_WIDTH]; MAX_HEIGHT],
    width: usize,
    height: usize,
) -> String {
    let mut out = String::new();
    for y in 0..height {
        for x in 0..width {
            match tiles[y][x] {
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
    out
}

/// Initialize an array of empty tiles created with [Tile::new]
pub fn create_tiles(width: usize, height: usize) -> Tiles {
    if width > MAX_WIDTH || height > MAX_HEIGHT {
        panic!("Board size too big! This version of the program has been compiled to support the maximum size of {:?}", (MAX_WIDTH, MAX_HEIGHT));
    }
    let mut tiles: Tiles = [[None; MAX_WIDTH]; MAX_HEIGHT];
    for x in 0..width {
        for y in 0..height {
            tiles[y][x] = Some(Tile {
                x,
                y,
                value: 0,
                ..Default::default()
            });
        }
    }
    return tiles;
}

/// Return the closest tile with the value of "mask" to the tile "t" in the given direction "dir",
/// if None is returned, no such tile was found.
pub fn get_closest_tile(
    t: Tile,
    viable_tiles: &Vec<Tile>,
    dir: Direction,
    mask: usize,
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
                    if r.value == i.value && r.merged_from.is_none() {
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
    return closest;
}

/// Return the farthest tile with the value of "mask" to the tile "t" in the given direction "dir",
/// if None is returned, no such tile was found.
pub fn get_farthest_tile(
    t: Tile,
    all_tiles: &Vec<Tile>,
    dir: Direction,
    mask: usize,
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

            if distance != 0 && distance > farthest_dist && i.value == mask {
                farthest = Some(*i);
                farthest_dist = distance;
            } else if distance != 0 && i.value != mask && distance < nearest_blocking {
                nearest_blocking = distance;
            }
        }
    }
    if nearest_blocking < farthest_dist {
        return None;
    }
    return farthest;
}

const MAX_MOVE_CHECKS: usize = 256;
#[derive(Serialize, Deserialize)]
pub struct MoveResult {
    pub possible: bool,
    pub tiles: Tiles,
    pub score_gain: usize,
}
/// Check if a move is possible in the direction "dir"
pub fn check_move(board: Board, dir: Direction) -> MoveResult {
    if dir == Direction::END {
        return MoveResult {
            possible: true,
            tiles: board.tiles,
            score_gain: 0,
        };
    }
    if !has_possible_moves(board) {
        return MoveResult {
            possible: false,
            tiles: board.tiles,
            score_gain: 0,
        };
    }

    let mut was_changed = false;

    // copy the current board and unset merged_from
    let mut tiles = board.tiles;
    for t in board.get_occupied_tiles() {
        tiles[t.y][t.x] = Some(Tile {
            merged_from: None,
            ..t
        })
    }

    let mut score = 0;

    // Merge
    let mut ids_checked_for_merge: Vec<usize> = vec![];
    for _ in 0..MAX_MOVE_CHECKS {
        let b = Board {
            tiles,
            height: board.height,
            width: board.width,
        };
        let occupied_tiles = b.get_occupied_tiles();
        let viable_tiles: Vec<Tile> = occupied_tiles
            .iter()
            .filter(|t| t.merged_from.is_none())
            .map(|t| *t)
            .collect();
        if let Some(t) = viable_tiles
            .iter()
            .find(|t| !ids_checked_for_merge.contains(&t.id))
        {
            let closest_opt = get_closest_tile(*t, &occupied_tiles, dir, t.value);
            if let Some(closest) = closest_opt {
                if t.value == closest.value && closest.merged_from.is_none() {
                    tiles[t.y][t.x] = Some(Tile::new(t.x, t.y, 0, None));

                    let merged: Tile = Tile {
                        x: closest.x,
                        y: closest.y,
                        value: closest.value * 2,
                        merged_from: Some([t.id, closest.id]),
                        ..Default::default()
                    };

                    score += merged.value;
                    tiles[closest.y][closest.x] = Some(merged);
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
        let b = Board {
            tiles,
            width: board.width,
            height: board.height,
        };
        let tiles_post = b.get_occupied_tiles();

        if let Some(t) = tiles_post.iter().find(|t| !moved_tiles.contains(&t.id)) {
            let all_tiles = b.get_all_tiles();
            let dir_to_use = dir;
            let farthest_free_opt = get_farthest_tile(*t, &all_tiles, dir_to_use, 0);

            if let Some(farthest_free) = farthest_free_opt {
                let new_tile: Tile = Tile {
                    x: farthest_free.x,
                    y: farthest_free.y,
                    ..*t
                };

                tiles[t.y][t.x] = Some(Tile::new(t.x, t.y, 0, None));
                tiles[farthest_free.y][farthest_free.x] = Some(new_tile);

                was_changed = true;
                moved_tiles = vec![];
            } else {
                moved_tiles.push(t.id);
            }
        } else {
            break;
        }
    }

    return MoveResult {
        possible: was_changed,
        tiles,
        score_gain: score,
    };
}

pub fn has_possible_moves(board: Board) -> bool {
    const NEIGHBOUR_DIRECTIONS: [Direction; 4] = [
        Direction::UP,
        Direction::RIGHT,
        Direction::DOWN,
        Direction::LEFT,
    ];
    if board.get_non_occupied_tiles().len() > 0 {
        return true;
    }
    for t in board.get_occupied_tiles() {
        for dir in NEIGHBOUR_DIRECTIONS {
            let (off_x, off_y) = (dir.get_x(), dir.get_y());
            let x: i64 = t.x as i64 + off_x;
            let y: i64 = t.y as i64 + off_y;
            if (x < 0 || y < 0) || (x as usize > board.width - 1 || y as usize > board.height - 1) {
                continue;
            }
            match board.tiles[y as usize][x as usize] {
                Some(neighbour) => {
                    if t.value == neighbour.value {
                        return true;
                    }
                }
                None => {}
            }
        }
    }
    false
}
