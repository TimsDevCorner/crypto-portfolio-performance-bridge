ALTER TABLE mexc_my_trades RENAME TO mexc_my_trades_old;

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
	created_at TEXT NOT NULL,
	updated_at TEXT,

    PRIMARY KEY (id)
) ;

INSERT INTO mexc_my_trades(
    symbol, id, order_id, order_list_id, price, qty,
    quote_qty, commission, commission_asset, time,
    is_buyer, is_maker, is_best_match, is_self_trade,
    client_order_id,
	
	created_at)
SELECT 
    symbol, id, order_id, order_list_id, price, qty,
    quote_qty, commission, commission_asset, time,
    is_buyer, is_maker, is_best_match, is_self_trade,
    client_order_id,
	
	CURRENT_TIMESTAMP
FROM mexc_my_trades_old;

DROP TABLE mexc_my_trades_old;

select * from mexc_my_trades;
