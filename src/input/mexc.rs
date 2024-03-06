use chrono::{TimeZone, Utc};
use futures::future::join_all;
use hmac::Mac;
use reqwest::Response;
use sqlx::{query, Pool, Sqlite};

use crate::data::{Amount, Application, Asset, Trade};

use super::{HmacSha256, InputError};

async fn request(url: &str) -> Result<Response, InputError> {
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

async fn request_signed(url: &str, parameters: &str) -> Result<Response, InputError> {
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

async fn get_symbols() -> Result<Vec<String>, InputError> {
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
    order_list_id: i64,
    price: String,
    qty: String,
    quote_qty: String,
    commission: String,
    commission_asset: String,
    time: i64,
    is_buyer: bool,
    is_maker: bool,
    is_best_match: bool,
    is_self_trade: bool,
    client_order_id: Option<String>,
}

async fn retrieve_and_save_trades_for_symbol(
    db: &Pool<Sqlite>,
    symbol: String,
) -> Result<(), InputError> {
    let trade_result = request_signed("myTrades", &format!("symbol={symbol}"))
        .await?
        .json::<Vec<MyTradesResult>>()
        .await?;

    for trade in trade_result {
        let exists = query!("SELECT id FROM mexc_my_trades WHERE id = $1", trade.id)
            .fetch_optional(db)
            .await?
            .is_some();

        if exists {
            continue;
        }

        query!(
            "INSERT INTO mexc_my_trades (		
                    symbol, id, order_id,
                    order_list_id, price, qty,
                    quote_qty, commission, commission_asset,
                    time, is_buyer, is_maker,
                    is_best_match, is_self_trade, client_order_id) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)",
            trade.symbol,
            trade.id,
            trade.order_id,
            trade.order_list_id,
            trade.price,
            trade.qty,
            trade.quote_qty,
            trade.commission,
            trade.commission_asset,
            trade.time,
            trade.is_buyer,
            trade.is_maker,
            trade.is_best_match,
            trade.is_self_trade,
            trade.client_order_id
        )
        .execute(db)
        .await?;
    }

    Ok(())
}

pub async fn get_all_trades(db: &Pool<Sqlite>) -> Result<Vec<Trade>, InputError> {
    let trades = query!("SELECT * FROM mexc_my_trades")
        .fetch_all(db)
        .await?
        .into_iter()
        .map(|row| {
            if !row.symbol.ends_with("USDT") {
                panic!("Unsupported symbol {}", row.symbol);
            }

            let crypto = Amount {
                amount: row.qty.parse().unwrap(),
                asset: Asset {
                    name: row.symbol.split("USDT").next().unwrap().to_string(),
                    contract_address: None,
                },
            };

            let usdt = Amount {
                amount: row.quote_qty.parse().unwrap(),
                asset: Asset {
                    name: "USDT".to_string(),
                    contract_address: None,
                },
            };

            let comission = Amount {
                amount: row.commission.parse().unwrap(),
                asset: Asset {
                    name: row.commission_asset,
                    contract_address: None,
                },
            };

            let (source, destination) = if row.is_buyer != 1 {
                // bought crypto
                (usdt, crypto)
            } else {
                // sold crypto
                (crypto, usdt)
            };

            Trade {
                application: Application("MEXC".to_string()),
                tx_id: row.id,
                source,
                destination,
                comission,
                timestamp: Utc.timestamp_millis_opt(row.time).unwrap(),
            }
        })
        .collect::<Vec<_>>();

    Ok(trades)
}

pub async fn gather_data(db: &Pool<Sqlite>) -> Result<(), InputError> {
    let symbols = get_symbols().await?;

    let retrieved_trades = symbols
        .into_iter()
        .map(|sym| retrieve_and_save_trades_for_symbol(db, sym));

    // The collect is necessary for easy error handling
    join_all(retrieved_trades)
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}
