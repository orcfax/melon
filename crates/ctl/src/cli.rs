use melon_core::cbor::ToCbor;
use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;

use crate::Command;

#[derive(clap::Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the unix socket
    #[arg(long, default_value = "/tmp/melon.sock")]
    pub socket: String,

    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub async fn run(&self) -> anyhow::Result<()> {
        let mut stream = UnixStream::connect(&self.socket)
            .await
            .expect("Missing socket?");
        let payload = self.command.to_cbor();
        stream.write_all(&payload).await?;
        println!("Sent");
        Ok(())
    }
}
