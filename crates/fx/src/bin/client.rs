use clap::Parser;
use std::time::Duration;

use melon_fx::PriceParams;

#[derive(Parser, Debug)]
#[command(author, version, about = "FX Polling Client")]
struct ClientArgs {
    /// Server URL
    #[arg(short, long, default_value = "http://127.0.0.1:3000/prices")]
    url: String,

    /// Polling interval in seconds
    #[arg(long, default_value_t = 0.5)]
    every: f64,

    /// Bullish bias (integer)
    #[arg(long)]
    bullish: Option<i32>,

    /// Bearish bias (integer)
    #[arg(long)]
    bearish: Option<i32>,

    /// Comma separated pairs to include (e.g. BTCUSD,ETHUSD)
    #[arg(long)]
    include: Option<String>,

    /// Comma separated pairs to exclude
    #[arg(long)]
    exclude: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = ClientArgs::parse();
    let client = reqwest::Client::new();

    let query = PriceParams {
        bullish: args.bullish,
        bearish: args.bearish,
        include: args.include,
        exclude: args.exclude,
    };

    println!("Polling {} every {}s...", args.url, args.every);

    let interval = Duration::from_secs_f64(args.every);

    loop {
        let start = tokio::time::Instant::now();

        match client.get(&args.url).query(&query).send().await {
            Ok(resp) => {
                if let Ok(prices) = resp.json::<Vec<(String, String, f64)>>().await {
                    println!(
                        "\n--- Market Tick [{:?}] ---",
                        chrono::Local::now().format("%H:%M:%S").to_string()
                    );
                    for (base, quote, price) in prices {
                        println!("{:<4} / {:<4} : {:>10.4}", base, quote, price);
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }

        let elapsed = start.elapsed();
        if elapsed < interval {
            tokio::time::sleep(interval - elapsed).await;
        }
    }
}
