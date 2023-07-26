//! Interface version 2
//!
//! Designed to be simpler and more flexible than [v1](crate::v1).
//!
//! See [SeededRecording](crate::v2::recording::SeededRecording) for more details.
pub mod io;
pub mod recording;
pub mod replay;

#[cfg(test)]
pub mod test_data;
