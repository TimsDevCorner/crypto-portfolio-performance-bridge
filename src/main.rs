#![deny(clippy::all)]

use clap::Parser;
use command_line_interface::Cli;
use command_line_interface::Command;
use input::InputError;
use sqlx::migrate;

pub mod command_line_interface;
pub mod data;
pub mod input;

#[macro_use]
extern crate dotenv_codegen;

#[tokio::main]
async fn main() -> Result<(), InputError> {
    // async fn main() {
    let cli = Cli::parse();

    let db = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect(dotenv!("DATABASE_URL"))
        .await
        .expect("Failed to connect to the database");

    migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    let result = match cli.command {
        Command::Fetch { exchange } => input::gather_data(&db, exchange).await,
        Command::Display {} => input::list_all_trades(&db).await,
        _ => {
            todo!()
        }
    };

    result.expect("Failed to gather data");

    Ok(())
}
