use std::sync::Arc;

use hmac::Hmac;
use sha2::Sha256;
use sqlx::{Pool, Sqlite};

use crate::{command_line_interface::Exchange, data::Trade};

pub mod coinbase;
pub mod mexc;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug)]
pub enum InputError {
    RequestError(Arc<reqwest::Error>),
    SqlError(Arc<sqlx::Error>),
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

async fn save_trades(db: Pool<Sqlite>, trades: Vec<Trade>) -> Result<(), InputError> {
    todo!()
}

pub async fn gather_data(db: &Pool<Sqlite>, exchange: Option<Exchange>) -> Result<(), InputError> {
    if let Some(exchange) = exchange {
        match exchange {
            Exchange::MEXC => mexc::gather_data(db).await?,
            Exchange::Coinbase => coinbase::gather_data(db).await?,
        }
    } else {
        mexc::gather_data(db).await?;
        coinbase::gather_data(db).await?;
        todo!()
    };

    Ok(())
}
