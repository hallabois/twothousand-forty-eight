use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IDAssignment {
    Simple,
    SimpleStateful,
    RandomStateful,
    RandomStatefulPositionBased,
}

impl Default for IDAssignment {
    fn default() -> Self {
        Self::SimpleStateful
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileIDAssigner;

impl TileIDAssigner {
    pub fn next_id(rng_state: &mut usize) -> usize {
        let id = *rng_state;
        *rng_state += 1;
        id
    }
}
