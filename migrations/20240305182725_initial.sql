
CREATE TABLE IF NOT EXISTS mexc_my_trades(
		symbol TEXT NOT NULL,
    id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    order_list_id INTEGER NOT NULL,
    price TEXT NOT NULL,
    qty TEXT NOT NULL,
    quote_qty TEXT NOT NULL,
    commission TEXT NOT NULL,
    commission_asset TEXT NOT NULL,
    time INTEGER NOT NULL,
    is_buyer INTEGER NOT NULL,
    is_maker INTEGER NOT NULL,
    is_best_match INTEGER NOT NULL,
    is_self_trade INTEGER NOT NULL,
    client_order_id TEXT,

		PRIMARY KEY (id)
) ;

    
