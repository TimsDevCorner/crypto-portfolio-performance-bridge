// use reqwest::Response;
//
// use super::{HmacSha256, InputError};
// use hmac::Mac;
//
// #[derive(Debug, serde::Deserialize)]
// pub struct CoinbaseResult<T> {
//     data: T,
// }
//
// #[derive(Debug, serde::Deserialize)]
// pub struct TimeResult {
//     epoch: u64,
// }
//
// pub async fn request(path: &str) -> Result<Response, InputError> {
//     let client = reqwest::Client::new();
//     let resp = client
//         .get(format!("https://api.coinbase.com{path}"))
//         .send()
//         .await?;
//
//     Ok(resp)
// }
//
// pub async fn get_server_time() -> Result<u64, InputError> {
//     Ok(request("/v2/time")
//         .await?
//         .json::<CoinbaseResult<TimeResult>>()
//         .await?
//         .data
//         .epoch)
// }
//
// pub async fn request_signed(path: &str, parameters: &str) -> Result<Response, InputError> {
//     let client = reqwest::Client::new();
//
//     let time = get_server_time().await?;
//     let parameters = if parameters.is_empty() {
//         "".to_string()
//     } else {
//         format!("?{parameters}")
//     };
//     let body = "";
//     let method = "GET";
//
//     let signed = format!("{time}{method}{path}{body}");
//     let mut mac = HmacSha256::new_from_slice(dotenv!("COINBASE_API_SECRET").as_bytes()).unwrap();
//     mac.update(signed.as_bytes());
//     let signed = mac.finalize().into_bytes();
//
//     let resp = client
//         .get(format!("https://api.coinbase.com{path}{parameters}"))
//         .header("CB-ACCESS-KEY", dotenv!("COINBASE_API_KEY"))
//         .header("CB-ACCESS-SIGN", format!("{:#01x}", signed))
//         .header("CB-ACCESS-TIMESTAMP", time.to_string())
//         .send()
//         .await?;
//
//     if resp.status() != 200 {
//         return Err(InputError::StatusError(resp.status().as_u16()));
//     }
//
//     Ok(resp)
// }
//
// #[derive(Debug, serde::Deserialize)]
// pub struct AccountResult {
//     id: String,
// }
//
// pub async fn get_accounts() -> Result<Vec<String>, InputError> {
//     let accounts = request_signed("/v2/accounts", "")
//         .await?
//         .json::<CoinbaseResult<Vec<AccountResult>>>()
//         .await?;
//
//     let accounts = accounts.data.into_iter().map(|x| x.id).collect();
//
//     Ok(accounts)
// }
//
// #[derive(Debug, serde::Deserialize)]
// pub struct AmountResult {
//     amount: String,
//     currency: String,
// }
//
// #[derive(Debug, serde::Deserialize)]
// pub struct TransactionResult {
//     id: String,
//     r#type: String, // "buy"
//     status: String, // "pendign"
//     amount: AmountResult,
//     native_amount: AmountResult,
//     created_at: String, //Iso String
//     updated_at: String, //Iso String
//     resource: String,   // "transaction"
// }
//
// pub async fn get_transactions(account: &str) -> Result<Vec<String>, InputError> {
//     let transactions = request_signed(&format!("/v2/accounts/{account}/transactions"), "")
//         .await?
//         .json::<CoinbaseResult<Vec<TransactionResult>>>()
//         .await?;
//
//     trades
//
//     Ok(vec![])
// }
//
// pub async fn gather_data() -> Result<(), InputError> {
//     let accounts = get_accounts().await?;
//     // let transactions
//
//     Ok(())
// }
