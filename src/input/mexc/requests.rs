use hmac::Mac;
use reqwest::Response;

use super::{HmacSha256, InputError};

pub async fn request(url: &str) -> Result<Response, InputError> {
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

pub async fn request_signed(url: &str, parameters: &str) -> Result<Response, InputError> {
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

pub async fn get_symbols() -> Result<Vec<String>, InputError> {
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
        "MNGLUSDT".to_string(),
    ];

    Ok(symbols)
}
