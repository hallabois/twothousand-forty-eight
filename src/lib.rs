//! a basic engine and move validator for the game 2048
//!
//! Includes wasm bindings generated with wasm_bindgen

#[allow(clippy::needless_range_loop)]
pub mod board;
pub mod direction;
pub mod random;
pub mod rules;
pub mod v1;
#[cfg(feature = "v2")]
pub mod v2;

#[cfg(feature = "wasm")]
pub mod wasm;

use board::tile;

pub fn get_random_tile_to_add(board: &mut board::Board) -> Option<tile::Tile> {
    use random::Pickable;

    let possible = board.get_non_occupied_tiles();
    if !possible.is_empty() {
        let t = possible.pick_lcg(&mut board.rng_state);

        let value = tile::Tile::random_value(&mut board.rng_state);

        return Some(tile::Tile::new(t.x, t.y, value, tile::InitialID::Id(t.id)));
    }
    None
}

pub fn add_random_to_board(board: &mut board::Board) {
    let possible_t = get_random_tile_to_add(board);
    match possible_t {
        None => {}
        Some(t) => {
            board.set_tile(t.x, t.y, t.value);
        }
    }
}
