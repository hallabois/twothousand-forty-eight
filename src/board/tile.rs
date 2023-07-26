use std::num::Wrapping;

use serde::{Deserialize, Serialize};

use crate::random::Pickable;

/// Tile is a basic representation of the tiles on the game board.
#[allow(clippy::derived_hash_with_manual_eq)]
#[derive(Debug, Copy, Clone, Eq, Serialize, Deserialize)]
#[cfg(feature = "wasm")]
#[derive(tsify::Tsify)]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
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

pub enum InitialID<'a> {
    Id(usize),
    AutoAssign(&'a mut usize),
}

fn next_id(id_counter: &mut usize) -> usize {
    let id = *id_counter;
    // use wrapping to allow overflow
    *id_counter = (Wrapping(*id_counter) + Wrapping(1)).0;
    id
}

impl From<usize> for InitialID<'_> {
    fn from(id: usize) -> Self {
        Self::Id(id)
    }
}

impl Tile {
    /// Create a new tile. A new unique identifier will be assigned to the generated tile if none was provided.
    pub fn new(x: usize, y: usize, value: usize, id: InitialID) -> Tile {
        Tile {
            x,
            y,
            value,
            id: Self::get_id(id),
            merged_from: None,
        }
    }

    /// Gives a json representation of the tile that is compatible with our anticheat systems
    pub fn to_json(&self) -> String {
        if self.value == 0 {
            return String::from("null");
        }
        serde_json::to_string(self).unwrap()
    }

    fn get_id(id: InitialID) -> usize {
        match id {
            InitialID::Id(id) => id,
            InitialID::AutoAssign(id_counter) => next_id(id_counter),
        }
    }

    pub fn random_value(seed: &mut usize) -> usize {
        let possible_values: [usize; 4] = [2, 2, 2, 4];
        *possible_values.pick_lcg(seed)
    }

    pub fn compare(a: &Self, b: &Self) -> bool {
        a.x == b.x && a.y == b.y && a.value == b.value
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.value == other.value
    }
}

impl std::hash::Hash for Tile {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.value.hash(state);
        // We don't hash the id or merged_from, as they are not relevant when comparing tiles
        //self.id.hash(state);
        //self.merged_from.hash(state);
    }
}
