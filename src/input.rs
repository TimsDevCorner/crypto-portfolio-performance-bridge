use std::sync::Arc;

use futures::join;
use hmac::Hmac;
use sha2::Sha256;
use sqlx::{Pool, Sqlite};

use crate::{command_line_interface::Exchange, data::Transaction};

pub mod coinbase;
pub mod mexc;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug)]
pub enum InputError {
    RequestError(Arc<reqwest::Error>),
    SqlError(Arc<sqlx::Error>),
    CsvError(Arc<csv::Error>),
    IoError(Arc<std::io::Error>),
    StatusError(u16),
}
impl From<reqwest::Error> for InputError {
    fn from(error: reqwest::Error) -> Self {
        InputError::RequestError(Arc::new(error))
    }
}
impl From<sqlx::Error> for InputError {
    fn from(error: sqlx::Error) -> Self {
        InputError::SqlError(Arc::new(error))
    }
}
impl From<csv::Error> for InputError {
    fn from(error: csv::Error) -> Self {
        InputError::CsvError(Arc::new(error))
    }
}
impl From<std::io::Error> for InputError {
    fn from(error: std::io::Error) -> Self {
        InputError::IoError(Arc::new(error))
    }
}

pub async fn gather_data(db: &Pool<Sqlite>, exchange: Option<Exchange>) -> Result<(), InputError> {
    if let Some(exchange) = exchange {
        match exchange {
            Exchange::MEXC => mexc::gather_data(db).await?,
            Exchange::Coinbase => coinbase::gather_data(db).await?,
        }
    } else {
        let mexc = mexc::gather_data(db);
        let coinbase = coinbase::gather_data(db);

        let result = join!(mexc, coinbase);
        result.0?;
        result.1?;
    };

    Ok(())
}

pub async fn get_all_trades(db: &Pool<Sqlite>) -> Result<Vec<Transaction>, InputError> {
    let mut trades = vec![];

    trades.append(&mut mexc::get_all_trades(db).await?);
    trades.append(&mut coinbase::get_all_trades(db).await?);

    Ok(trades)
}

pub async fn list_all_trades(db: &Pool<Sqlite>) -> Result<(), InputError> {
    let trades = get_all_trades(db).await?;
    let json = serde_json::to_string(&trades).unwrap();

    println!("{}", json);

    Ok(())
}
