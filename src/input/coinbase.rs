use reqwest::Response;
use serde::de::DeserializeOwned;
use sqlx::{query, Pool, Sqlite};

use crate::data::{Amount, Application, Asset, Trade};

use super::{HmacSha256, InputError};
use hmac::Mac;

#[derive(Debug, serde::Deserialize)]
pub struct Pagination {
    next_uri: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CoinbaseResult<T> {
    data: T,
    pagination: Option<Pagination>,
}

#[derive(Debug, serde::Deserialize)]
pub struct TimeResult {
    epoch: u64,
}

pub async fn request(path: &str) -> Result<Response, InputError> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://api.coinbase.com{path}"))
        .send()
        .await?;

    Ok(resp)
}

pub async fn get_server_time() -> Result<u64, InputError> {
    Ok(request("/v2/time")
        .await?
        .json::<CoinbaseResult<TimeResult>>()
        .await?
        .data
        .epoch)
}

pub async fn request_signed(path: &str) -> Result<Response, InputError> {
    let client = reqwest::Client::new();
    let parameters = "";

    let time = get_server_time().await?;
    let parameters = if parameters.is_empty() {
        "".to_string()
    } else {
        format!("?{parameters}")
    };
    let body = "";
    let method = "GET";

    let signed = format!("{time}{method}{path}{body}");
    let mut mac = HmacSha256::new_from_slice(dotenv!("COINBASE_API_SECRET").as_bytes()).unwrap();
    mac.update(signed.as_bytes());
    let signed = mac.finalize().into_bytes();

    let resp = client
        .get(format!("https://api.coinbase.com{path}{parameters}"))
        .header("CB-ACCESS-KEY", dotenv!("COINBASE_API_KEY"))
        .header("CB-ACCESS-SIGN", format!("{:#01x}", signed))
        .header("CB-ACCESS-TIMESTAMP", time.to_string())
        .send()
        .await?;

    if resp.status() != 200 {
        return Err(InputError::StatusError(resp.status().as_u16()));
    }

    Ok(resp)
}

pub async fn request_all_pages<T: DeserializeOwned>(
    signed: bool,
    path: &str,
) -> Result<Vec<T>, InputError> {
    let mut result = vec![];
    let mut path = path.to_string();

    loop {
        let response = if signed {
            request_signed(&path).await?
        } else {
            request(&path).await?
        };
        let mut response = response.json::<CoinbaseResult<Vec<T>>>().await?;
        result.append(&mut response.data);

        if let Some(pagination) = response.pagination {
            if let Some(next_uri) = pagination.next_uri {
                path = next_uri;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    Ok(result)
}

#[derive(Debug, serde::Deserialize)]
pub struct AccountResult {
    id: String,
}

pub async fn get_account_ids() -> Result<Vec<String>, InputError> {
    let accounts = request_all_pages::<AccountResult>(true, "/v2/accounts").await?;

    let accounts = accounts.into_iter().map(|x| x.id).collect();

    Ok(accounts)
}

#[derive(Debug, serde::Deserialize)]
pub struct AmountResult {
    amount: String,
    currency: String,
}
#[derive(Debug, Clone, serde::Deserialize)]
pub struct NetworkResult {
    status: String,
    name: Option<String>,
}
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ToResult {
    id: Option<String>,
    resource: String,
    resource_path: Option<String>,
}
#[derive(Debug, serde::Deserialize)]
pub struct DetailsResult {
    title: String,
    subtitle: String,
}
#[derive(Debug, serde::Deserialize)]
pub struct TransactionResult {
    id: String,
    r#type: String,
    status: String,
    amount: AmountResult,
    native_amount: AmountResult,
    description: Option<String>,
    created_at: String,
    updated_at: String,
    resource: String,
    resource_path: String,
    network: Option<NetworkResult>,
    to: Option<ToResult>,
    details: DetailsResult,
}

pub async fn retrieve_and_save_transactions(
    db: &Pool<Sqlite>,
    account: &str,
) -> Result<Vec<TransactionResult>, InputError> {
    let transactions = request_all_pages::<TransactionResult>(
        true,
        &format!("/v2/accounts/{account}/transactions"),
    )
    .await?;

    for transaction in transactions {
        let exists = query!(
            "SELECT id FROM coinbase_transactions WHERE id = $1",
            transaction.id
        )
        .fetch_optional(db)
        .await?
        .is_some();

        if exists {
            continue;
        }

        let network_status = transaction.network.clone().map(|n| n.status);
        let network_name = transaction.network.clone().map(|n| n.name);
        let to_id = transaction.to.clone().map(|t| t.id);
        let to_resource = transaction.to.clone().map(|t| t.resource);
        let to_resource_path = transaction.to.clone().map(|t| t.resource_path);
        query!(
            "INSERT INTO coinbase_transactions (
                id, type, status,
                amount_amount, amount_currency,
                native_amount_amount, native_amount_currency,
                description, created_at, updated_at,
                resource, resource_path,
                network_status, network_name,
                to_id, to_resource, to_resource_path,
                details_title, details_subtitle
            ) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)",
            transaction.id,
            transaction.r#type,
            transaction.status,
            transaction.amount.amount,
            transaction.amount.currency,
            transaction.native_amount.amount,
            transaction.native_amount.currency,
            transaction.description,
            transaction.created_at,
            transaction.updated_at,
            transaction.resource,
            transaction.resource_path,
            network_status,
            network_name,
            to_id,
            to_resource,
            to_resource_path,
            transaction.details.title,
            transaction.details.subtitle
        ).execute(db).await?;
    }
    Ok(vec![])
}

pub async fn gather_data(db: &Pool<Sqlite>) -> Result<(), InputError> {
    let account_ids = get_account_ids().await?;

    let mut transactions = vec![];
    for account in account_ids {
        transactions.append(&mut retrieve_and_save_transactions(db, &account).await?);
    }

    Ok(())
}

pub async fn get_all_trades(db: &Pool<Sqlite>) -> Result<Vec<Trade>, InputError> {
    let mut transactions_except_trades =
        query!("SELECT * FROM coinbase_transactions WHERE type != 'trade'")
            .fetch_all(db)
            .await?
            .into_iter()
            .filter(|row| match row.r#type.as_ref() {
                "buy" => row.amount_currency != "EUR",
                "sell" => row.amount_currency != "EUR",

                "earn_payout" => true,

                "send" => false,
                "fiat_deposit" => false,

                _ => unimplemented!("Coinbase type '{}' is not implemented", row.r#type),
            })
            .map(|row| match row.r#type.as_ref() {
                "buy" => Trade {
                    application: Application("Coinbase".to_string()),
                    tx_id: row.id,
                    source: Some(Amount {
                        amount: row.native_amount_amount.parse().unwrap(),
                        asset: Asset {
                            name: row.native_amount_currency,
                            contract_address: None,
                        },
                    }),
                    destination: Amount {
                        amount: row.amount_amount.parse().unwrap(),
                        asset: Asset {
                            name: row.amount_currency,
                            contract_address: None,
                        },
                    },
                    comission: None,
                    timestamp: row.created_at.parse().unwrap(),
                },
                "sell" => Trade {
                    application: Application("Coinbase".to_string()),
                    tx_id: row.id,
                    source: Some(Amount {
                        amount: row.amount_amount.parse::<f64>().unwrap() * -1.0,
                        asset: Asset {
                            name: row.amount_currency,
                            contract_address: None,
                        },
                    }),
                    destination: Amount {
                        amount: row.native_amount_amount.parse::<f64>().unwrap() * -1.0,
                        asset: Asset {
                            name: row.native_amount_currency,
                            contract_address: None,
                        },
                    },
                    comission: None,
                    timestamp: row.created_at.parse().unwrap(),
                },
                "earn_payout" => Trade {
                    application: Application("Coinbase".to_string()),
                    tx_id: row.id,
                    source: None,
                    destination: Amount {
                        amount: row.amount_amount.parse().unwrap(),
                        asset: Asset {
                            name: row.amount_currency,
                            contract_address: None,
                        },
                    },
                    comission: None,
                    timestamp: row.created_at.parse().unwrap(),
                },
                _ => {
                    todo!()
                }
            })
            .collect::<Vec<Trade>>();

    let mut trades = vec![];
    trades.append(&mut transactions_except_trades);

    let transactions_trades =
        query!("SELECT * FROM coinbase_transactions WHERE type = 'trade' ORDER BY created_at")
            .fetch_all(db)
            .await?;
    let mut source = None;
    let mut destination = None;

    for trade in transactions_trades {
        if trade.native_amount_currency != "EUR" {
            unimplemented!(
                "Coinbase trade with currency {}",
                trade.native_amount_currency
            );
        }

        if trade.amount_amount.contains('-') {
            source = Some((
                Amount {
                    amount: trade.amount_amount.parse::<f64>().unwrap() * -1.0,
                    asset: Asset {
                        name: trade.amount_currency,
                        contract_address: None,
                    },
                },
                trade.native_amount_amount.parse::<f64>().unwrap() * -1.0,
            ));
        } else {
            destination = Some((
                Amount {
                    amount: trade.amount_amount.parse().unwrap(),
                    asset: Asset {
                        name: trade.amount_currency,
                        contract_address: None,
                    },
                },
                trade.native_amount_amount.parse::<f64>().unwrap(),
            ));
        }

        if source.is_some() && destination.is_some() {
            let comission = Some(Amount {
                amount: source.clone().unwrap().1 - destination.clone().unwrap().1,
                asset: Asset {
                    name: trade.native_amount_currency,
                    contract_address: None,
                },
            });

            trades.push(Trade {
                application: Application("Coinbase".to_string()),
                tx_id: trade.id,
                source: source.map(|(amount, _native)| amount),
                destination: destination.expect("").0,
                comission,
                timestamp: trade.created_at.parse().unwrap(),
            });

            source = None;
            destination = None;
        }
    }

    Ok(trades)
}
