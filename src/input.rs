// use std::sync::Arc;
//
// use chrono::{DateTime, Utc};
// use hmac::Hmac;
// use sha2::Sha256;
//
// pub mod coinbase;
// pub mod mexc;
//
// type HmacSha256 = Hmac<Sha256>;
//
// #[derive(Debug)]
// pub enum InputError {
//     RequestError(Arc<reqwest::Error>),
//     StatusError(u16),
// }
// impl From<reqwest::Error> for InputError {
//     fn from(error: reqwest::Error) -> Self {
//         InputError::RequestError(Arc::new(error))
//     }
// }
//
// pub async fn gather_all_data() -> Result<(), InputError> {
//     // let mexc_data = mexc::gather_data().await?;
//     let coinbase_data = coinbase::gather_data().await?;
//
//     println!("{:?}", coinbase_data);
//
//     Ok(())
// }
