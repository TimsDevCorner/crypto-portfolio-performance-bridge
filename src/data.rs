use chrono::{DateTime, Utc};

/// A struct representing an place where assets can be stored
/// e.g. a CEX, DEX, Bank or wallet
pub enum Wallet {
    Bank,
    Mexc,
    Coinbase,
    MetaMask(String),
    CoinbaseWallet(String),
    Solfare(String),
    Compass(String),
    Alephium(String),
}

/// A struct representing an exchange, where assets can be traded
pub enum Exchange {
    Mexc,
    Coinbase,
    UniSwap,
    SushiSwap,
}

/// A struct representing an asset
/// An asset is a currency or token that can be traded
pub struct Asset {
    pub symbol: String,
    pub contract_address: Option<String>,
}

/// A struct representing an amount of an asset
pub struct Amount {
    pub quantity: f64,
    pub asset: Asset,
}

/// A struct representing an Bank, Wallet or CEX with corresponding balances
pub struct Account {
    pub name: Wallet,
    pub balances: Vec<Amount>,
}

/// A struct representing a trade
/// A trade is a transaction where one asset is exchanged for another
pub struct Trade {
    /// The exchange where the trade took place
    pub exchange: Exchange,
    /// The unique identifier of the trade, defined by the exchange
    pub id: String,
    /// The asset that was sold
    pub from: Amount,
    /// The Asset that was bought
    pub to: Amount,
    /// The Asset paid as commission for the trade
    pub commission: Amount,
    /// The time the trade took place
    pub time: DateTime<Utc>,
}

/// A struct representing a transfer
/// A transfer is a transaction where one asset is sent from one account to another
pub struct Transfer {
    /// The unique identifier of the transfer, defined by the Bank, Wallet or CEX
    pub id: String,
    /// The source of the transfer
    pub from: Wallet,
    /// The destination of the transfer
    pub to: Wallet,
    /// The asset that was transfered
    pub amount: Amount,
    /// The time the transfer took place
    pub commission: Option<Amount>,
    /// The time the transfer took place
    pub time: DateTime<Utc>,
}
