//! a basic engine and move validator for the game 2048
//!
//! Includes wasm bindings generated with wasm_bindgen

#[allow(clippy::needless_range_loop)]
pub mod board;
pub mod direction;
pub mod random;
pub mod rules;
pub mod unified;
pub mod v1;
pub mod v2;

#[cfg(feature = "wasm")]
pub mod wasm;
