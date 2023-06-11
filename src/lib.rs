//! a basic engine and move validator for the game 2048
//!
//! Includes wasm bindings generated with wasm_bindgen

#[allow(clippy::needless_range_loop)]
pub mod board;
pub mod direction;
pub mod parser;
pub mod random;
pub mod recording;
pub mod validator;

#[cfg(feature = "wasm")]
pub mod wasm;

use board::tile;

pub fn get_random_tile_to_add(board: &board::Board, seed: Option<usize>) -> Option<tile::Tile> {
    use random::Pickable;

    let possible = board.get_non_occupied_tiles();
    if !possible.is_empty() {
        let seed = seed.unwrap_or_else(|| {
            let total_value = board.get_total_value();
            possible.len() + total_value
        });
        let t = possible.pick_lcg(seed);

        let value = tile::Tile::random_value(seed);

        return Some(tile::Tile {
            x: t.x,
            y: t.y,
            value,
            ..Default::default()
        });
    }
    None
}

pub fn add_random_to_board(board: &mut board::Board, seed: Option<usize>) {
    let possible_t = get_random_tile_to_add(board, seed);
    match possible_t {
        None => {}
        Some(t) => {
            board.set_tile(t.x, t.y, t.value);
        }
    }
}

// Tests
#[cfg(test)]
pub mod tests;
