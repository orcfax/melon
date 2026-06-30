use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use melon_bus::{Bus, Event};
use melon_core::{Bytes, Domain, Endorsement, Signer, domain};
use melon_fx::{Client, Currency, PriceResponse};

use crate::statement::{FeedId, Rational, Statement};
use crate::{Config, Error};

const BAD_ENCODE_SEVERITY: u8 = 10;
const BAD_ID_SEVERITY: u8 = 10;

// ---------------------------------------------------------------------------
// Price cache
// ---------------------------------------------------------------------------

struct PriceCache {
    prices: PriceResponse,
    updated_at: tokio::time::Instant,
}

impl PriceCache {
    fn new(prices: PriceResponse) -> Self {
        Self {
            prices,
            updated_at: tokio::time::Instant::now(),
        }
    }

    fn is_stale(&self, threshold: Duration) -> bool {
        self.updated_at.elapsed() > threshold
    }

    fn lookup(&self, base: &Currency, quote: &Currency) -> Option<f64> {
        self.prices.iter().find_map(|(b, q, p)| {
            if base == b && quote == q {
                Some(*p)
            } else {
                None
            }
        })
    }

    fn to_statements(&self) -> Vec<Statement> {
        to_statements(self.prices.clone())
    }
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

pub struct Service {
    config: Config,
    signer: Arc<Signer>,
    client: Client,
    cache: Option<PriceCache>,
    bus: Arc<Bus>,
}

impl Service {
    pub fn new(config: Config, signer: Arc<Signer>, bus: Arc<Bus>) -> Self {
        let client = Client::new(&config.fx_url);
        Self {
            config,
            signer,
            client,
            cache: None,
            bus,
        }
    }

    pub async fn run(mut self) -> Result<(), Error> {
        self.poll().await;

        let mut rx = self.bus.subscribe();
        let mut poll_tick = tokio::time::interval(self.config.poll_every);

        loop {
            tokio::select! {
                Ok(event) = rx.recv() => {
                    match event {
                        Event::Proposed { endorsement, data } => {
                            self.on_proposed(endorsement, data)?;
                        }
                        Event::OpApp { .. } => {
                            self.on_op_app()?;
                        }
                        _ => {}
                    }
                }
                _ = poll_tick.tick() => {
                    self.poll().await;
                }
            }
        }
    }

    // -- polling ------------------------------------------------------------

    async fn poll(&mut self) {
        match self.client.poll(&self.config.query).await {
            Ok(response) => {
                tracing::debug!("prices updated");
                self.cache = Some(PriceCache::new(response));
            }
            Err(e) => {
                tracing::warn!("price poll failed: {e}");
            }
        }
    }

    // -- proposing ----------------------------------------------------------

    fn on_op_app(&self) -> Result<(), Error> {
        let Some(cache) = &self.cache else {
            tracing::warn!("no price data, cannot propose");
            return Ok(());
        };

        if cache.is_stale(self.config.staleness_threshold) {
            tracing::warn!("price data stale, cannot propose");
            return Ok(());
        }

        let statements = cache.to_statements();
        for statement in statements {
            self.propose(&statement)?;
        }

        Ok(())
    }

    fn propose(&self, statement: &Statement) -> Result<(), Error> {
        let point = statement.to_g1();
        let sig = self.signer.sign(&point);
        let vk = self.signer.verifying_key();
        let id = statement.id();

        let data = minicbor::to_vec(statement).expect("Infallible");

        let endorsement = Endorsement {
            id,
            kind: domain::Kind::App,
            key: vk,
            sig,
        };

        self.bus.publish(Event::Proposed {
            endorsement,
            data: Bytes::from(data),
        });

        Ok(())
    }

    // -- endorsing ----------------------------------------------------------

    fn on_proposed(&self, endorsement: Endorsement, data: Bytes) -> Result<(), Error> {
        if endorsement.kind != domain::Kind::App {
            return Ok(());
        }

        if endorsement.key == self.signer.verifying_key() {
            return Ok(());
        }

        let Some(statement) = minicbor::decode::<Statement>(&data.0).ok() else {
            let severity = BAD_ENCODE_SEVERITY;
            self.bus.publish(Event::BadMessage {
                endorsement,
                severity,
            });
            return Ok(());
        };

        if statement.id() != endorsement.id {
            let severity = BAD_ID_SEVERITY;
            self.bus.publish(Event::BadMessage {
                endorsement,
                severity,
            });
            return Ok(());
        }

        if !self.verify(&statement) {
            tracing::warn!("proposal failed validation, not endorsing");
            return Ok(());
        }

        self.endorse(statement)?;
        Ok(())
    }

    fn endorse(&self, statement: Statement) -> Result<(), Error> {
        let point = statement.to_g1();
        let sig = self.signer.sign(&point);
        let vk = self.signer.verifying_key();

        self.bus.publish(Event::Endorsed(Endorsement {
            id: statement.id(),
            kind: domain::Kind::App,
            key: vk,
            sig,
        }));

        Ok(())
    }

    // -- validation ---------------------------------------------------------

    fn verify(&self, statement: &Statement) -> bool {
        let Some(cache) = &self.cache else {
            return false;
        };

        if cache.is_stale(self.config.staleness_threshold) {
            return false;
        }

        if now().abs_diff(statement.created_at()) > self.config.staleness_threshold {
            return false;
        }

        let Some((base, quote)) = feed_id_to_pair(statement.feed_id()) else {
            return false;
        };

        let Some(price) = cache.lookup(&base, &quote) else {
            return false;
        };

        let body = statement.body();
        let diff = (price - body.as_f64()).abs();
        let sum = (price + body.as_f64()).abs();
        if sum == 0.0 {
            return diff == 0.0;
        }
        let deviation = 2.0 * diff / sum;
        deviation <= self.config.deviation_threshold
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn now() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}

fn feed_id_to_pair(feed_id: &FeedId) -> Option<(Currency, Currency)> {
    use std::str::FromStr;
    let s = format!("{}", feed_id);
    let inner = s.strip_prefix("CER/")?.strip_suffix("/V3")?;
    let (base, quote) = inner.split_once('-')?;
    Some((
        Currency::from_str(base).ok()?,
        Currency::from_str(quote).ok()?,
    ))
}

fn to_statements(prices: PriceResponse) -> Vec<Statement> {
    prices
        .into_iter()
        .filter_map(|(base, quote, price)| to_statement(base, quote, price).ok())
        .collect()
}

fn to_statement(base: Currency, quote: Currency, price: f64) -> Result<Statement, String> {
    let id_string = format!(
        "CER/{}-{}/V3",
        base.to_string().to_uppercase(),
        quote.to_string().to_uppercase(),
    );
    let feed_id = FeedId::from(id_string.as_str());
    let body = Rational::from_f64(price, 1_000_000);
    Ok(Statement::new(feed_id, now(), body))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        let response: PriceResponse = vec![
            (Currency::BTC, Currency::USD, 65000.50),
            (Currency::ETH, Currency::EUR, 0.00012345),
        ];
        let statements = to_statements(response);
        assert_eq!(statements.len(), 2);
    }

    #[test]
    fn test_feed_id_to_pair() {
        let feed_id = FeedId::from("CER/ADA-USD/V3");
        let (base, quote) = feed_id_to_pair(&feed_id).unwrap();
        assert_eq!(base, Currency::ADA);
        assert_eq!(quote, Currency::USD);
    }
}
