use chrono::{DateTime, Utc};

pub mod mexc;

#[derive(Debug, Clone)]
pub struct Trade {
    source: String,
    symbol: String,
    id: String,
    order_id: String,
    price: f64,
    qty: f64,
    total: f64,
    commission: f64,
    commission_asset: String,
    time: DateTime<Utc>,
}

pub async fn gather_all_data() -> Result<(), Box<dyn std::error::Error>> {
    let resp = mexc::gather_data().await;

    println!("{:?}", resp);

    Ok(())
}
