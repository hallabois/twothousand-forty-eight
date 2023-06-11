use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IDAssignment {
    Simple,
    Seeded(usize),
    SimpleControlled(TileIDAssigner),
    SeededControlled(TileIDAssigner),
}

impl Default for IDAssignment {
    fn default() -> Self {
        Self::Simple
    }
}

impl From<Option<usize>> for IDAssignment {
    fn from(value: Option<usize>) -> Self {
        match value {
            Some(seed) => Self::Seeded(seed),
            None => Self::default(),
        }
    }
}

impl From<IDAssignment> for Option<usize> {
    fn from(value: IDAssignment) -> Self {
        match value {
            IDAssignment::Simple => None,
            IDAssignment::Seeded(seed) => Some(seed),
            IDAssignment::SimpleControlled(s) => Some(s.counter),
            IDAssignment::SeededControlled(s) => Some(s.counter),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileIDAssigner {
    counter: usize,
}

impl TileIDAssigner {
    pub fn new(counter: usize) -> Self {
        Self { counter }
    }
    pub fn next_id(&mut self) -> usize {
        let id = self.counter;
        self.counter += 1;
        id
    }
}

impl Default for TileIDAssigner {
    fn default() -> Self {
        Self::new(0)
    }
}
