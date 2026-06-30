use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
)]
#[strum(ascii_case_insensitive)]
pub enum Currency {
    BTC,
    ETH,
    ADA,
    USD,
    EUR,
    GBP,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrencyPair {
    pub base: Currency,
    pub quote: Currency,
    pub price: f64,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct PriceParams {
    pub bullish: Option<i32>,
    pub bearish: Option<i32>,
    pub exclude: Option<String>,
    pub include: Option<String>,
}

/// A type alias for the standard response format: [base, quote, price]
pub type PriceResponse = Vec<(Currency, Currency, f64)>;
