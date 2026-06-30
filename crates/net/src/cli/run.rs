use std::sync::Arc;

use melon_bus::Bus;
use tracing::info;

use crate::{Config, Service};

/// Standalone Version ///
#[derive(Debug, Clone, clap::Args)]
pub struct NetRunStandalone {
    #[clap(flatten)]
    core: melon_core::cli::Run,
}

impl NetRunStandalone {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config_fp = &self.core.config()?;
        let config: melon_core::Config = melon_core::config::read(config_fp)?;
        let net_config = Config::try_from(config)?;
        let bus = Arc::new(Bus::new(1024));
        let service = Service::new(net_config, bus.clone()).await?;
        let handle = tokio::spawn(async move { service.run().await });

        tokio::select! {
            res = handle => info!("Net service exited: {:?}", res),
        };
        Ok(())
    }
}
