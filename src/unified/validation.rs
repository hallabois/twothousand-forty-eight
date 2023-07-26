pub const MAX_ALLOWED_BREAKS: usize = 3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ValidationResult {
    /// The maximum score reached during the run, 0 if the run is not valid
    pub score: usize,
    /// The score at the end of the run, may not match score if a break was used near the end of the run
    pub score_end: usize,
    /// Error margin on the score, in case the last move was not recorded
    pub score_margin: usize,
    /// Amount of breaks used
    pub breaks: usize,
    /// When those breaks happened
    pub break_positions: [Option<usize>; MAX_ALLOWED_BREAKS],
}

pub trait Validatable {
    type Error;
    fn validate(&self) -> Result<ValidationResult, Self::Error>;
}
