// use std::collections::HashMap;
//
// use chrono::{DateTime, Utc};

// Where can crypto can be stored?
pub struct Wallet {
    pub name: String,
    pub balaces: Vec<WalletBalance>,
}
pub struct Cex {
    pub name: String,
    pub balances: Vec<CexBalance>,
}

pub struct WalletBalance {
    pub asset: Asset,
    pub network: String,
    pub balance: f64,
}
pub struct CexBalance {
    pub asset: Asset,
    pub balance: f64,
}

pub struct Asset {
    pub name: String,
    pub contract_address: Option<String>,
    pub main_network: String,
}

// how can be interacted with a Cex
// how can be interacted with a Wallet
