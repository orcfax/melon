use std::sync::Arc;

use melon_bus::Bus;
use melon_core::domain;
use tracing::info;

#[derive(Debug, Clone, clap::Args)]
pub struct SetRunStandalone {
    #[clap(flatten)]
    core: melon_core::cli::Run,
}

/// FIXME :: This should be in the config.
fn dsts(k: domain::Kind) -> &'static [u8] {
    match k {
        domain::Kind::Block => b"BLOCK_DST",
        domain::Kind::Sys => b"SYS_DST",
        domain::Kind::App => b"APP_DST",
    }
}

impl SetRunStandalone {
    pub async fn run(&self) -> anyhow::Result<()> {
        // Config
        let config_fp = &self.core.config()?;
        let config: melon_core::Config = melon_core::config::read(config_fp)?;

        // Bus
        let bus = Arc::new(Bus::new(1024));

        // Net
        let net_config = melon_net::Config::try_from(config.clone())?;
        let net_service = melon_net::Service::new(net_config, bus.clone()).await?;
        let net_handle = tokio::spawn(async move { net_service.run().await });

        // Set
        let set_service = crate::Service::new(config.manifest.members.clone(), dsts, bus.clone());
        let set_handle = tokio::spawn(async move { set_service.run().await });

        tokio::select! {
            res = net_handle => info!("Net service exited: {:?}", res),
            res = set_handle => info!("Set service exited: {:?}", res),
        };
        Ok(())
    }
}
