#[cfg(feature = "wasm")]
use serde::{Serialize, Deserialize};
#[cfg(feature = "tile_id")]
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
#[cfg_attr(not(feature = "tile_id"), derive(PartialEq))]
pub struct Tile{
    pub x: usize,
    pub y: usize,
    pub value: usize,
    pub merged: bool,
    #[cfg(feature = "tile_id")]
    pub id: usize,
}

#[cfg(feature = "tile_id")]
impl PartialEq for Tile{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.value == other.value && self.merged == other.merged
    }
}

impl Tile{
    pub fn new(x: usize, y: usize, value: usize, merged: bool) -> Tile{
        Tile{
            x: x,
            y: y,
            value: value,
            merged: merged,
            #[cfg(feature = "tile_id")]
            id: Tile::get_new_id(),
        }
    }
    pub fn oispahalla_serialize(&self) -> String{
        if self.value == 0{
            return String::from("null");
        }
        return format!("{{\"position\":{{\"x\":{},\"y\":{}}},\"value\":{}}}", self.y, self.x, self.value);
    }
    #[cfg(feature = "tile_id")]
    fn get_new_id() -> usize {
        static COUNTER:AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed)
    }
}