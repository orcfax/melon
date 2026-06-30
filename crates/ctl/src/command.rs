use melon_bus::Event;
use melon_core::Id;
use minicbor::{Decode, Encode};

/// FIXME!!
#[derive(clap::Subcommand, Debug, Clone, Encode, Decode)]
pub enum Command {
    #[n(0)]
    App {
        #[n(0)]
        data: String,
    },
    #[n(1)]
    Status,
    #[n(2)]
    Shutdown,
}

impl From<Command> for Event {
    fn from(value: Command) -> Self {
        match value {
            Command::App { data } => Event::OpApp {
                data: data.into_bytes().into(),
            },
            Command::Status => Event::OpStatus,
            Command::Shutdown => Event::OpShutdown,
        }
    }
}

#[derive(Clone, Debug, clap::Args, Encode, Decode)]
pub struct FindArgs {
    #[arg(long, value_parser = parse_find_args)]
    #[n(0)]
    id: Id,
}

fn parse_find_args(s: &str) -> Result<Id, String> {
    s.parse::<Id>().map_err(|e| e.to_string())
}

#[derive(Clone, Debug, clap::Args, Encode, Decode)]
pub struct PrintQcArgs {
    /// Print as hex-encoded CBOR instead of pretty output.
    #[arg(long)]
    #[n(0)]
    pub hex: bool,
}
