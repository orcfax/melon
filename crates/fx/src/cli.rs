use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "FX Server Configuration")]
pub struct ServerCli {
    /// Address to bind the server to
    #[arg(short, long, default_value = "127.0.0.1:3000")]
    pub addr: String,

    /// Initial market volatility (0.01 = 1%)
    #[arg(short, long, default_value_t = 0.01)]
    pub volatility: f64,
}
