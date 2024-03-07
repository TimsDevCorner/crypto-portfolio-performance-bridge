use chrono::{DateTime, Utc};
use serde::Serialize;

pub struct Network(
    /// The name of the network
    pub String,
);

/// An Wallet is any place an crypto currency or fiat can be stored
/// e.g. CEX, Wallets, Banks, etc
pub struct Wallet {
    /// Name of the wallet
    /// e.g. Coinbase, MetaMask, etc
    pub name: String,
    /// The list of all balances contained in the wallet
    pub balaces: Vec<WalletBalance>,
}

/// WalletBalance is the amount of an asset in a wallet
pub struct WalletBalance {
    /// The amount of the asset in the wallet
    pub amount: Amount,
    /// The network the asset is on, if it is stored on a wallet
    /// A CEX or a Bank doesn't have a multiple networks
    pub network: Option<Network>,
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
    /// The stored, traded, etc asset, e.g. BTC
    pub asset: Asset,
}

/// An Application is any place an crypto currency or fiat can be traded or bridged
/// e.g. Uniswap, PancakeSwap, etc
#[derive(Debug, Clone, Serialize)]
pub struct Application(pub String);

/// A Bridge is a transaction that moves an asset from one network to another
pub struct Bridge {
    /// The application that made the bridge
    pub application: Application,
    /// The unique id of the transaction
    pub tx_id: String,

    /// The network the asset is coming from
    pub source: Network,
    /// The network the asset is going to
    pub destination: Network,
    /// The amount of the asset being bridged
    pub amount: Amount,
    /// The comission paid for the bridge
    pub comission: Amount,

    /// The timestamp the trasaction took place
    pub timestamp: DateTime<Utc>,
}

/// A Trade is a transaction where one asset is exchanged for another
#[derive(Debug, Clone, Serialize)]
pub struct Trade {
    /// The application that made the trade
    pub application: Application,
    /// The unique id of the transaction
    pub tx_id: String,

    /// The asset being sold
    /// In case of airdrops or bank transfer of fiat, the source is None
    pub source: Option<Amount>,
    /// The asset being bought
    pub destination: Amount,
    /// The comission paid for the trade
    pub comission: Option<Amount>,

    /// The timestamp the trasaction took place
    pub timestamp: DateTime<Utc>,
}

/// A Transfer is a transaction where an asset is moved from one wallet to another
pub struct Transfer {
    /// The unique id of the transfer
    pub tx_id: String,

    /// The wallet the asset is coming from
    pub source: Wallet,
    /// The wallet the asset is going to
    pub destination: Wallet,

    /// The amount of the asset being transferred
    pub amount: Amount,
    /// The comission paid for the transfer
    pub comission: Amount,

    /// The timestamp the transfer took place
    pub timestamp: DateTime<Utc>,
}
