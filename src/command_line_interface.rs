use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// fetch data from exchanges, defaulting to all
    Fetch {
        #[arg(short, long)]
        /// specify the exchange to fetch data from
        exchange: Option<Exchange>,
    },
    /// Display data from exchanges, defaulting to all
    Display {
        #[arg(short, long)]
        /// specify the exchange to display data from
        exchange: Option<Exchange>,
    },
    /// Export data from exchanges, defaulting to all
    Export {
        #[arg(short, long)]
        /// specify the exchange to export data from
        exchange: Option<Exchange>,
    },
}

#[derive(Subcommand)]
enum FetchOptions {
    All,
    /// the list of exchanges to fetch data from
    Exchange {
        exchange: Exchange,
    },
}

#[derive(ValueEnum, Clone)]
enum Exchange {
    Coinbase,
    MEXC,
}
