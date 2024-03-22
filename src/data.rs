use chrono::{DateTime, Utc};
use serde::Serialize;

pub type Network = String;

/// An ValueStore is any place an crypto currency or fiat can be stored
/// e.g. CEX, Wallets, Banks, etc
#[derive(Debug, Clone, Serialize)]
pub enum ValueStore {
    Cex(String),
    Wallet {
        /// Name of the wallet
        /// e.g. MetaMask, QubicWallet etc
        name: String,
        /// The network the wallet is on
        network: Network,
        /// The public address of the wallet
        address: String,
    },
}

/// An Asset is any crypto currency or fiat
#[derive(Debug, Clone, Serialize)]
pub struct Asset {
    /// The unique ticker of the asset
    pub name: String,
    /// Contract address of the asset, if it has one
    pub contract_address: Option<String>,
}

/// An Amount is the amount and asset used in other structs
#[derive(Debug, Clone, Serialize)]
pub struct Amount {
    /// The amount of the asset stored, traded, etc
    pub amount: f64,
    /// The stored, traded, etc asset, e.g. BTC, USD, EUR
    pub asset: Asset,
}

/// An Application is any place an crypto currency or fiat can be traded or bridged
/// e.g. Uniswap, PancakeSwap, etc
#[derive(Debug, Clone, Serialize)]
pub struct Application(pub String);

#[derive(Debug, Clone, Serialize)]
pub enum Transaction {
    Trade(Trade),
    /// An Airdrop is a transaction where an asset is given to an account for free
    Airdrop(Airdrop),
    /// A Bridge is a transaction that moves an asset from one network to another
    Bridge(Bridge),
}

#[derive(Debug, Clone, Serialize)]
pub struct Comission {
    pub amount: Amount,
    pub usd_amount: f64,
}

/// A Trade is a transaction where one asset is exchanged for another
#[derive(Debug, Clone, Serialize)]
pub struct Trade {
    /// The application that made the trade
    pub application: Application,
    /// The unique id of the transaction
    pub tx_id: String,

    /// The asset being sold
    pub source: Amount,
    /// The asset being bought
    pub destination: Amount,
    /// The comission paid for the trade
    pub comission: Option<Comission>,
    /// The value of the whole trade in the fiat currency, comission included
    pub usd_amount: f64,

    /// The timestamp the trasaction took place
    pub timestamp: DateTime<Utc>,
}

///
#[derive(Debug, Clone, Serialize)]
pub struct Airdrop {
    /// The unique id of the transaction
    pub tx_id: String,

    /// The amount being airdropped
    pub amount: Amount,
    /// The value of the whole airdrop in the usd
    pub usd_amount: f64,

    /// The timestamp the trasaction took place
    pub timestamp: DateTime<Utc>,

    /// The reason for the airdrop
    pub note: String,
}

/// A Bridge is a transaction that moves an asset from one network to another
#[derive(Debug, Clone, Serialize)]
pub struct Bridge {
    /// The application that made the bridge
    pub application: Application,
    /// The unique id of the transaction
    pub tx_id: String,

    /// The network the asset is coming from
    pub source: ValueStore,
    /// The network the asset is going to
    pub destination: ValueStore,
    /// The amount of the asset being bridged
    pub amount: Amount,
    /// The comission paid for the bridge
    pub comission: Amount,

    /// The timestamp the trasaction took place
    timestamp: DateTime<Utc>,
}
