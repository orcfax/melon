use melon_core::Members;
use serde::{Deserialize, Serialize};
// ---------------------------------------------------------------------------
// Config
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub members: Members,
}
