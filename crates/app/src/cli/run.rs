use std::sync::Arc;

use melon_bus::Bus;
use melon_core::{Domain, domain};
use tracing::info;

use crate::Statement;

/// Standalone Version ///
#[derive(Debug, Clone, clap::Args)]
pub struct App {
    #[clap(flatten)]
    config: melon_core::cli::Run,
}

/// FIXME :: This should be in the config.
fn dsts(k: domain::Kind) -> &'static [u8] {
    match k {
        domain::Kind::Block => b"BLOCK_DST",
        domain::Kind::Sys => b"SYS_DST",
        domain::Kind::App => Statement::DST,
    }
}

impl App {
    pub async fn run(&self) -> anyhow::Result<()> {
        // Config
        let config_fp = &self.config.config()?;
        let config: super::Config = melon_core::config::read(config_fp)?;

        // Bus
        let bus = Arc::new(Bus::new(1024));

        // Ctl
        let ctl_config = melon_ctl::Config::from_config_path(config_fp.to_str().unwrap());
        let ctl_service = melon_ctl::Service::new(ctl_config, bus.clone()).await?;
        let ctl_handle = tokio::spawn(async move { ctl_service.run().await });

        // Net
        let net_config = melon_net::Config::try_from(config.core.clone())?;
        let net_service = melon_net::Service::new(net_config, bus.clone()).await?;
        let net_handle = tokio::spawn(async move { net_service.run().await });

        let set_service =
            melon_set::Service::new(config.core.manifest.members.clone(), dsts, bus.clone());
        let set_handle = tokio::spawn(async move { set_service.run().await });

        let signer = Arc::new(melon_core::Signer::new(
            config.core.secrets.member.clone().unwrap(),
        ));

        let app_service =
            crate::Service::new(crate::config::Config::default(), signer, bus.clone());
        let app_handle = tokio::spawn(async move { app_service.run().await });

        tokio::select! {
            res = ctl_handle => info!("Ctl service exited: {:?}", res),
            res = net_handle => info!("Net service exited: {:?}", res),
            res = set_handle => info!("Set service exited: {:?}", res),
            res = app_handle => info!("App service exited: {:?}", res),
        };
        Ok(())
    }
}
