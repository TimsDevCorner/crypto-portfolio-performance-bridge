use std::sync::Arc;

use chrono::DateTime;
use futures::future::join_all;
use hmac::{Hmac, Mac};
use reqwest::Response;
use sha2::Sha256;

use crate::input::Trade;

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub enum MexcError {
    RequestError(Arc<reqwest::Error>),
    StatusError(u16),
}
impl From<reqwest::Error> for MexcError {
    fn from(error: reqwest::Error) -> Self {
        MexcError::RequestError(Arc::new(error))
    }
}

async fn request(url: &str) -> Result<Response, MexcError> {
    let client = reqwest::Client::new();

    let resp = client
        .get(format!("https://api.mexc.com/api/v3/{url}",))
        .header("X-MEXC-APIKEY", dotenv!("MEXC_ACCESS_KEY"))
        .header("Content-Type", "application/json");

    Ok(resp.send().await?)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct TimeResult {
    server_time: u64,
}

async fn request_signed(url: &str, parameters: &str) -> Result<Response, MexcError> {
    let client = reqwest::Client::new();

    let now = request("time")
        .await?
        .json::<TimeResult>()
        .await?
        .server_time;

    let parameters = if parameters.is_empty() {
        format!("timestamp={now}")
    } else {
        format!("timestamp={now}&{parameters}")
    };

    let mut mac = HmacSha256::new_from_slice(dotenv!("MEXC_SECRET_KEY").as_bytes()).unwrap();
    mac.update(parameters.as_bytes());
    let signature = mac.finalize().into_bytes();

    let resp = client
        .get(format!(
            "https://api.mexc.com/api/v3/{url}?signature={:#01x}&{parameters}",
            signature
        ))
        .header("X-MEXC-APIKEY", dotenv!("MEXC_ACCESS_KEY"))
        .header("Content-Type", "application/json")
        .send()
        .await?;

    if resp.status() != 200 {
        panic!(
            "Status error {}, {}",
            resp.status().as_u16(),
            resp.text().await?
        );
    }

    Ok(resp)
}

async fn get_symbols() -> Result<Vec<String>, MexcError> {
    // Using the API to gather symbols leads to a 403 error due to too many requests

    let symbols = vec![
        "ALPHUSDT".to_string(),
        "APTUSDT".to_string(),
        "AZEROUSDT".to_string(),
        "BNBUSDT".to_string(),
        "CAKEUSDT".to_string(),
        "COTIUSDT".to_string(),
        "DYMUSDT".to_string(),
        "ETHUSDT".to_string(),
        "KASUSDT".to_string(),
        "MINAUSDT".to_string(),
        "ONDOUSDT".to_string(),
        "RVFUSDT".to_string(),
        "TRIASUSDT".to_string(),
        "WELSHUSDT".to_string(),
        "XAIUSDT".to_string(),
        "ZEPHUSDT".to_string(),
        "QNTUSDT".to_string(),
    ];

    Ok(symbols)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct MyTradesResult {
    symbol: String,
    id: String,
    order_id: String,
    price: String,
    qty: String,
    quote_qty: String,
    commission: String,
    commission_asset: String,
    time: u64,
}

async fn get_trades_for_symbol(symbol: String) -> Result<Vec<Trade>, MexcError> {
    let trade_result = request_signed("myTrades", &format!("symbol={symbol}"))
        .await?
        .json::<Vec<MyTradesResult>>()
        .await?;

    let trades: Vec<_> = trade_result
        .into_iter()
        .map(|trade| Trade {
            source: "MEXC".to_string(),
            symbol: trade.symbol,
            id: trade.id,
            order_id: trade.order_id,
            price: trade.price.parse().unwrap(),
            qty: trade.qty.parse().unwrap(),
            total: trade.quote_qty.parse().unwrap(),
            commission: trade.commission.parse().unwrap(),
            commission_asset: trade.commission_asset,
            time: DateTime::from_timestamp_millis(trade.time as i64).unwrap(),
        })
        .collect();

    Ok(trades)
}

pub async fn gather_data() -> Result<Vec<Trade>, MexcError> {
    let symbols = get_symbols().await?;

    let trades = symbols.into_iter().map(get_trades_for_symbol);
    let trades = join_all(trades).await;
    let trades: Result<Vec<_>, _> = trades.into_iter().collect();
    let trades: Vec<_> = trades?.into_iter().flatten().collect();

    Ok(trades)
}
