pub mod input;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = input::gather_all_data().await?;

    Ok(())
}
