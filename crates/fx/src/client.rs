use crate::{PriceParams, PriceResponse};

pub struct Client {
    http: reqwest::Client,
    base_url: String,
    last_state: Option<PriceResponse>,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Self {
            http: reqwest::Client::new(),
            base_url: base_url.to_string(),
            last_state: None,
        }
    }

    /// Performs a single poll and updates internal state
    pub async fn poll(&mut self, params: &PriceParams) -> Result<PriceResponse, reqwest::Error> {
        let response = self
            .http
            .get(&self.base_url)
            .query(params)
            .send()
            .await?
            .error_for_status()?
            .json::<PriceResponse>()
            .await?;

        self.last_state = Some(response.clone());
        Ok(response)
    }

    pub fn get_last_state(&self) -> &Option<PriceResponse> {
        &self.last_state
    }
}
