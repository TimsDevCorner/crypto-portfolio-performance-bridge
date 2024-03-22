se chrono::{NaiveDate, NaiveTime};
use csv::Writer;
use sqlx::{Pool, Sqlite};

use crate::{
    data::{Airdrop, Asset, Trade, Transaction},
    input::{self, InputError},
};

#[derive(Debug, Clone, serde::Serialize)]
enum ExportTradeType {
    Buy,
    Sell,
}

#[derive(Debug, serde::Serialize)]
struct ExportTransaction {
    pub application: String,
    pub tx_id: String,

    /// EUR or USD
    pub currency: String,
    pub account: String,
    pub asset: String,
    pub ticker: String,

    pub r#type: ExportTradeType,

    pub crypto_amount: f64,
    pub fiat_amount: f64,
    pub comission_amount: f64,

    pub note: String,

    pub date: NaiveDate,
    pub time: NaiveTime,
}

fn is_usd(asset: &Asset) -> bool {
    asset.name == "USD"
}

fn map_trade(trade: Trade) -> Vec<ExportTransaction> {
    let currency: String;
    let asset: String;
    let ticker: String;
    let crypto_amount: f64;
    let fiat_amount: f64;
    let comission_amount: f64;

    if is_usd(&trade.source.asset) {
        currency = trade.source.asset.name;

        asset = trade.destination.asset.name;
        ticker = asset.clone();

        crypto_amount = trade.destination.amount;
        comission_amount = trade.comission.map(|com| com.amount).unwrap_or(0.0);
        fiat_amount = trade.source.amount - comission_amount;

        vec![ExportTransaction {
            application: trade.application.clone().0,
            tx_id: trade.tx_id.clone(),
            currency: currency.clone(),
            account: currency.clone(),
            asset: asset.clone(),
            ticker: ticker.clone(),
            r#type: ExportTradeType::Buy,
            crypto_amount,
            fiat_amount,
            comission_amount,
            note: "".to_string(),
            date: trade.timestamp.date_naive(),
            time: trade.timestamp.time(),
        }]
    } else if is_usd(&trade.destination.asset) {
        currency = trade.destination.clone().asset.name;
        asset = trade.source.asset.name;
        ticker = asset.clone();

        crypto_amount = trade.source.amount;
        comission_amount = trade.comission.map(|com| com.amount).unwrap_or(0.0);
        fiat_amount = trade.destination.amount - comission_amount;

        vec![ExportTransaction {
            application: trade.application.clone().0,
            tx_id: trade.tx_id.clone(),
            currency: currency.clone(),
            account: currency.clone(),
            asset: asset.clone(),
            ticker: ticker.clone(),
            r#type: ExportTradeType::Sell,
            crypto_amount,
            fiat_amount,
            comission_amount,
            note: "".to_string(),
            date: trade.timestamp.date_naive(),
            time: trade.timestamp.time(),
        }]
    } else {
        let comission = trade.comission.map(|com| com.usd_amount).unwrap_or(0.0);

        vec![
            ExportTransaction {
                application: trade.application.clone().0,
                tx_id: trade.tx_id.clone(),
                currency: "USD".to_string(),
                account: "USD".to_string(),
                asset: trade.source.asset.clone().name,
                ticker: trade.source.asset.name,
                r#type: ExportTradeType::Sell,
                crypto_amount: trade.source.amount,
                fiat_amount: trade.usd_amount - comission,
                comission_amount: comission,
                note: "".to_string(),
                date: trade.timestamp.date_naive(),
                time: trade.timestamp.time(),
            },
            ExportTransaction {
                application: trade.application.0,
                tx_id: trade.tx_id,
                currency: "USD".to_string(),
                account: "USD".to_string(),
                asset: trade.destination.clone().asset.name,
                ticker: trade.destination.asset.name,
                r#type: ExportTradeType::Buy,
                crypto_amount: trade.destination.amount,
                fiat_amount: trade.usd_amount - comission,
                comission_amount: 0.0,
                note: "".to_string(),
                date: trade.timestamp.date_naive(),
                time: trade.timestamp.time(),
            },
        ]
    }
}

fn map_airdrop(airdrop: Airdrop) -> Vec<ExportTransaction> {
    vec![ExportTransaction {
        application: airdrop.note.clone(),
        tx_id: airdrop.tx_id,
        currency: "USD".to_string(),
        account: "USD".to_string(),
        asset: airdrop.amount.clone().asset.name,
        ticker: airdrop.amount.asset.name,
        r#type: ExportTradeType::Buy,
        crypto_amount: airdrop.amount.amount,
        // As this is an airdrop, the actually paid amount is 0
        // It needs to be 0.01, otherwise portfolio performance will claim an error
        fiat_amount: 0.01,
        comission_amount: 0.0,
        note: "".to_string(),
        date: airdrop.timestamp.date_naive(),
        time: airdrop.timestamp.time(),
    }]
}

fn map_transaction(transaction: Transaction) -> Vec<ExportTransaction> {
    match transaction {
        Transaction::Trade(trade) => map_trade(trade),
        Transaction::Airdrop(airdrop) => map_airdrop(airdrop),
        Transaction::Bridge(_) => todo!(),
    }
}

pub async fn export_data(db: &Pool<Sqlite>) -> Result<(), InputError> {
    let trades = input::get_all_trades(db)
        .await?
        .into_iter()
        .flat_map(map_transaction)
        .collect::<Vec<_>>();

    let mut wtr = Writer::from_path("trades.csv")?;

    for trade in trades {
        wtr.serialize(trade)?;
    }

    wtr.flush()?;

    println!("Data exported to trades.csv");

    Ok(())
}
