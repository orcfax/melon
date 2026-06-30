use melon_bus::{Bus, Event};
use melon_core::cbor::FromCbor;
use std::sync::Arc;
use std::{fs, io};
use tokio::io::AsyncReadExt;
use tokio::net::{UnixListener, UnixStream};
use tracing::{error, warn};

use crate::Command;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Io :: {0}")]
    Io(#[from] io::Error),
    #[error("Misc :: {0}")]
    Misc(String),
}

pub struct Service {
    socket_path: String,
    bus: Arc<Bus>,
    listener: UnixListener,
}

impl Service {
    pub async fn new(config: super::Config, bus: Arc<Bus>) -> Result<Self, Error> {
        let path = config.socket.clone();
        let _ = fs::remove_file(&path);
        let listener = UnixListener::bind(&path)?;
        Ok(Self {
            socket_path: path,
            bus,
            listener,
        })
    }

    pub async fn run(self) -> Result<(), Error> {
        loop {
            let (stream, _) = self.listener.accept().await?;
            let bus = Arc::clone(&self.bus);
            tokio::spawn(async move {
                if let Err(e) = Self::handle_socket(stream, bus).await {
                    error!(err = %e, "[ctl] socket error");
                }
            });
        }
    }

    async fn handle_socket(mut stream: UnixStream, bus: Arc<Bus>) -> Result<(), Error> {
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).await?;

        match Command::from_cbor(&buffer) {
            Ok(cmd) => bus.publish(Event::from(cmd)),
            Err(e) => {
                warn!(err = ?e, "[ctl] malformed command");
            }
        }

        Ok(())
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.socket_path);
    }
}
