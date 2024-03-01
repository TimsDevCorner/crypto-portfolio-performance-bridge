#![deny(clippy::all)]

pub mod input;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    input::gather_all_data().await?;

    Ok(())
}
