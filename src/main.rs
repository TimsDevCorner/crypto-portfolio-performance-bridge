#![deny(clippy::all)]

use clap::Parser;
use command_line_interface::Cli;
// use input::InputError;

pub mod command_line_interface;
pub mod data;
pub mod input;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
// async fn main() -> Result<(), InputError> {
async fn main() {
    let cli = Cli::parse();

    // input::gather_all_data().await?;
}
