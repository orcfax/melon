use axum::{
    Json, Router,
    extract::{Query, State},
    routing::get,
};
use rand::Rng;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::model::{Currency, CurrencyPair, PriceParams, PriceResponse};

pub struct MarketState {
    pub pairs: Vec<CurrencyPair>,
    pub volatility: f64,
}

impl MarketState {
    pub fn new(pairs: Vec<(Currency, Currency, f64)>, volatility: f64) -> Self {
        let pairs = pairs
            .into_iter()
            .map(|(base, quote, price)| CurrencyPair { base, quote, price })
            .collect();
        Self { pairs, volatility }
    }

    pub fn update_prices(&mut self) {
        let mut rng = rand::rng();
        for pair in &mut self.pairs {
            let movement = rng.random_range(-self.volatility..self.volatility);
            pair.price *= 1.0 + movement;
            if pair.price < 0.0001 {
                pair.price = 0.0001;
            }
        }
    }
}

pub struct Server {
    state: Arc<Mutex<MarketState>>,
}

impl Server {
    pub fn new(initial_pairs: Vec<(Currency, Currency, f64)>, volatility: f64) -> Self {
        Self {
            state: Arc::new(Mutex::new(MarketState::new(initial_pairs, volatility))),
        }
    }

    pub async fn run(self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let app = Router::new()
            .route("/prices", get(Self::get_prices_handler))
            .with_state(self.state);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        println!("FX Server listening on http://{}", addr);
        axum::serve(listener, app).await?;
        Ok(())
    }

    async fn get_prices_handler(
        Query(params): Query<PriceParams>,
        State(state): State<Arc<Mutex<MarketState>>>,
    ) -> Json<PriceResponse> {
        let mut market = state.lock().await;
        market.update_prices();

        let bias = (params.bullish.unwrap_or(0) - params.bearish.unwrap_or(0)) as f64 * 0.001;

        let include_list: Vec<String> = params
            .include
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_uppercase())
            .filter(|s| !s.is_empty())
            .collect();

        let exclude_list: Vec<String> = params
            .exclude
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_uppercase())
            .filter(|s| !s.is_empty())
            .collect();

        let result = market
            .pairs
            .iter()
            .filter(|p| {
                let symbol = format!("{}{}", p.base, p.quote);
                if !include_list.is_empty() && !include_list.contains(&symbol) {
                    return false;
                }
                if exclude_list.contains(&symbol) {
                    return false;
                }
                true
            })
            .map(|p| (p.base, p.quote, p.price + bias))
            .collect();

        Json(result)
    }
}
