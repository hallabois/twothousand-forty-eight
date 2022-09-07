#[cfg(feature = "serde_derive")]
use serde::{Serialize, Deserialize};
#[cfg(feature = "tile_id")]
use std::sync::atomic::{AtomicUsize, Ordering};

/// Tile is a basic representation of the tiles on the game board.
/// 
/// If the feature "tile_id" is enabled, all the tiles are assigned a unique id, which is preserved in the history (excluding merges)
#[derive(Debug, Copy, Clone, Eq, Hash)]
#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[cfg_attr(not(feature = "tile_id"), derive(PartialEq))]
pub struct Tile{

    /// x coordinate of the tile, usize is always greater than zero
    pub x: usize,

    /// y coordinate of the tile, usize is always greater than zero
    pub y: usize,

    /// value of the tile, usually in a power of two, e.g. 2, 4, 8, 16, 32...
    pub value: usize,

    /// a variable used in internal calculations, should be false
    pub merged: bool,

    /// a unique identifier for the tile, not preserved when tiles merge together
    #[cfg(feature = "tile_id")]
    pub id: usize,

    #[cfg(feature = "tile_merged_from")]
    pub merged_from: Option<[usize; 2]>
}

#[cfg(feature = "tile_id")]
impl PartialEq for Tile{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.value == other.value && self.merged == other.merged
    }
}

impl Tile{

    /// Create a new tile. If the "tile_id" feature is enabled, a new unique identifier will be assigned to the generated tile.
    pub fn new(x: usize, y: usize, value: usize, merged: bool) -> Tile{
        Tile{
            x,
            y,
            value,
            merged,
            #[cfg(feature = "tile_id")]
            id: Tile::get_new_id(),
            #[cfg(feature = "tile_merged_from")]
            merged_from: None
        }
    }

    /// Gives a json representation of the tile that is compatible with our anticheat systems
    #[cfg(feature = "serde_derive")]
    pub fn to_json(&self) -> String{
        if self.value == 0 {
            return String::from("null");
        }
        serde_json::to_string(self).unwrap()
    }

    /// Provides a new identifier upon every call, essentially just incrementing the previous by one.
    #[cfg(feature = "tile_id")]
    fn get_new_id() -> usize {
        static COUNTER:AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}