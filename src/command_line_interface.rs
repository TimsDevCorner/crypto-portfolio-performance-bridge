use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// fetch data from exchanges, defaulting to all
    Fetch {
        #[arg(short, long)]
        /// specify the exchange to fetch data from
        exchange: Option<Exchange>,
    },
    /// Display data from exchanges, defaulting to all
    Display,
    /// Export data from exchanges, defaulting to all
    Export,
}

#[derive(Subcommand)]
pub enum FetchOptions {
    All,
    /// the list of exchanges to fetch data from
    Exchange {
        exchange: Exchange,
    },
}

#[derive(ValueEnum, Clone)]
pub enum Exchange {
    Coinbase,
    MEXC,
}
