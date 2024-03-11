use chrono::{DateTime, Utc};
use csv::Writer;
use sqlx::{Pool, Sqlite};

use crate::{
    data::Trade,
    input::{self, InputError},
};

#[derive(Debug, serde::Serialize)]
pub struct ExportTrade {
    pub application: String,
    pub tx_id: String,

    pub source_amount: Option<f64>,
    pub source_currency: Option<String>,

    pub destination_amount: f64,
    pub destination_currency: String,

    pub comission_amount: Option<f64>,
    pub comission_currency: Option<String>,

    pub timestamp: DateTime<Utc>,
}

fn map_trade(trade: Trade) -> ExportTrade {
    ExportTrade {
        application: trade.application.0,
        tx_id: trade.tx_id,

        source_amount: trade.source.clone().map(|amount| amount.amount),
        source_currency: trade.source.map(|amount| amount.asset.name),

        destination_amount: trade.destination.amount,
        destination_currency: trade.destination.asset.name,

        comission_amount: trade.comission.clone().map(|amount| amount.amount),
        comission_currency: trade.comission.map(|amount| amount.asset.name),

        timestamp: trade.timestamp,
    }
}

pub async fn export_data(db: &Pool<Sqlite>) -> Result<(), InputError> {
    let trades = input::get_all_trades(db)
        .await?
        .into_iter()
        .map(map_trade)
        .collect::<Vec<_>>();

    let mut wtr = Writer::from_path("trades.csv")?;

    for trade in trades {
        wtr.serialize(trade)?;
    }

    wtr.flush()?;

    println!("Data exported to trades.csv");

    Ok(())
}
