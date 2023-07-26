use serde::{Deserialize, Serialize};

use crate::board::Board;

use super::validation::ValidationResult;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg(feature = "wasm")]
#[derive(tsify::Tsify)]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct HistoryReconstruction {
    pub validation_data: ValidationResult,
    pub history: Vec<Board>,
}

pub trait Reconstructable {
    type ReconstructionError;
    fn reconstruct(&self) -> Result<HistoryReconstruction, Self::ReconstructionError>;
}
