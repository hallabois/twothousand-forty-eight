use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::random::Pickable;

/// Tile is a basic representation of the tiles on the game board.
#[derive(Debug, Copy, Clone, Eq, Hash, Serialize, Deserialize)]
pub struct Tile {
    /// x coordinate of the tile, usize is always greater than zero
    pub x: usize,

    /// y coordinate of the tile, usize is always greater than zero
    pub y: usize,

    /// value of the tile, usually in a power of two, e.g. 2, 4, 8, 16, 32...
    pub value: usize,

    /// a unique identifier for the tile, copied to the new tile's merged_from property on merge
    pub id: usize,

    pub merged_from: Option<[usize; 2]>,
}

impl Tile {
    /// Create a new tile. If the "tile_id" feature is enabled, a new unique identifier will be assigned to the generated tile if none was provided.
    pub fn new(x: usize, y: usize, value: usize, id: Option<usize>) -> Tile {
        Tile {
            x,
            y,
            value,
            id: id.unwrap_or(Self::get_new_id()),
            ..Default::default()
        }
    }

    /// Gives a json representation of the tile that is compatible with our anticheat systems
    pub fn to_json(&self) -> String {
        if self.value == 0 {
            return String::from("null");
        }
        serde_json::to_string(self).unwrap()
    }

    /// Provides a new identifier upon every call, essentially just incrementing the previous by one.
    fn get_new_id() -> usize {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }

    pub fn random_value(seed: usize) -> usize {
        let possible_values: [usize; 4] = [2, 2, 2, 4];
        *possible_values.pick_lcg(seed)
    }
}

impl Default for Tile {
    fn default() -> Self {
        let id = Self::get_new_id();
        let value = Self::random_value(id);
        Self {
            x: 0,
            y: 0,
            value,
            id,
            merged_from: None,
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.value == other.value
    }
}
