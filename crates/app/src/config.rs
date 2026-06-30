use serde::{Deserialize, Serialize};

use std::time::Duration;

use melon_fx::PriceParams;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// fx URL
    pub fx_url: String,
    /// Query params
    pub query: PriceParams,
    /// Frequency
    pub poll_every: Duration,
    /// Refuse to endorse any older than.
    pub staleness_threshold: Duration,
    /// Refuse to endorse any deviation greater than
    pub deviation_threshold: f64,
}

impl Config {
    pub fn default() -> Self {
        Self {
            fx_url: "http://127.0.0.1:3000/prices".to_string(),
            query: Default::default(),
            poll_every: Duration::from_secs(3),
            staleness_threshold: Duration::from_secs(10),
            deviation_threshold: 0.02,
        }
    }
}
