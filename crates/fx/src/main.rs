use clap::Parser;
use melon_fx::{Cli, Currency, Server};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let initial_data = vec![
        (Currency::BTC, Currency::USD, 62000.0),
        (Currency::ETH, Currency::USD, 3200.0),
        (Currency::EUR, Currency::USD, 1.08),
        (Currency::ADA, Currency::USD, 0.29),
    ];

    let server = Server::new(initial_data, cli.volatility);

    if let Err(e) = server.run(&cli.addr).await {
        eprintln!("Fatal server error: {}", e);
    }
}
