use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::random::Pickable;

use super::tile_id_assigner::{IDAssignment, TileIDAssigner};

/// Tile is a basic representation of the tiles on the game board.
#[allow(clippy::derived_hash_with_manual_eq)]
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

pub enum InitialID<'a> {
    Id(usize),
    Strategy(IDAssignment, &'a mut usize),
}
impl From<usize> for InitialID<'_> {
    fn from(id: usize) -> Self {
        Self::Id(id)
    }
}
/*
impl Default for InitialID<'_> {
    fn default() -> Self {
        Self::Strategy(IDAssignment::default(), 0)
    }
}
impl From<Option<usize>> for InitialID<'_> {
    fn from(id: Option<usize>) -> Self {
        match id {
            Some(id) => Self::Id(id),
            None => Self::default(),
        }
    }
} */
impl<'a> From<(IDAssignment, &'a mut usize)> for InitialID<'a> {
    fn from(state: (IDAssignment, &'a mut usize)) -> Self {
        let (id_assignment_strategy, rng_state) = state;
        Self::Strategy(id_assignment_strategy, rng_state)
    }
}

impl Tile {
    /// Create a new tile. If the "tile_id" feature is enabled, a new unique identifier will be assigned to the generated tile if none was provided.
    pub fn new(x: usize, y: usize, value: usize, id: InitialID) -> Tile {
        Tile {
            x,
            y,
            value,
            id: Self::get_id(id, x, y),
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

    fn get_id(id: InitialID, x: usize, y: usize) -> usize {
        match id {
            InitialID::Id(id) => id,
            InitialID::Strategy(id_assignment_strategy, rng_state) => {
                Self::get_new_id_from_assignment_strategy(id_assignment_strategy, rng_state, x, y)
            }
        }
    }
    fn get_new_id_from_assignment_strategy(
        assignment_strategy: IDAssignment,
        rng_state: &mut usize,
        x: usize,
        y: usize,
    ) -> usize {
        match assignment_strategy {
            IDAssignment::Simple => {
                // Provides a new identifier upon every call, incrementing the previous by one.
                static COUNTER: AtomicUsize = AtomicUsize::new(1);
                COUNTER.fetch_add(1, Ordering::Relaxed)
            }
            IDAssignment::SimpleStateful => TileIDAssigner::next_id(rng_state),
            IDAssignment::RandomStateful => {
                crate::random::lcg_sane(TileIDAssigner::next_id(rng_state))
            }
            IDAssignment::RandomStatefulPositionBased => {
                crate::random::lcg_sane(*rng_state + x + y)
            }
        }
    }

    pub fn random_value(seed: usize) -> usize {
        let possible_values: [usize; 4] = [2, 2, 2, 4];
        *possible_values.pick_lcg(seed)
    }
}

impl Default for Tile {
    fn default() -> Self {
        let id_assignment_strategy = IDAssignment::default();
        let id = Self::get_id(InitialID::Strategy(id_assignment_strategy, &mut 0), 0, 0);
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
