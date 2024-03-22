use chrono::{TimeZone, Utc};
use futures::future::join_all;
use sqlx::{query, Pool, Sqlite};

use crate::data::{Amount, Application, Asset, Comission, Trade, Transaction};

use self::requests::{get_symbols, request_signed};

use super::InputError;

pub mod requests;
pub mod trades;
pub mod transactions;

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
                    is_best_match, is_self_trade, client_order_id,
                    created_at) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, CURRENT_TIMESTAMP)",
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

pub async fn get_all_trades(db: &Pool<Sqlite>) -> Result<Vec<Transaction>, InputError> {
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

            let comission = if row.commission == "0" {
                None
            } else {
                Some(Comission {
                    amount: Amount {
                        amount: row.commission.parse().unwrap(),
                        asset: Asset {
                            name: row.commission_asset,
                            contract_address: None,
                        },
                    },
                    usd_amount: row.commission.parse().unwrap(),
                })
            };

            let (source, destination) = if row.is_buyer != 1 {
                // sold crypto
                (crypto, usdt)
            } else {
                // bought crypto
                (usdt, crypto)
            };

            Transaction::Trade(Trade {
                application: Application("MEXC".to_string()),
                tx_id: row.id,
                source,
                destination,
                comission,
                usd_amount: row.quote_qty.parse().unwrap(),
                timestamp: Utc.timestamp_millis_opt(row.time).unwrap(),
            })
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
