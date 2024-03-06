
CREATE TABLE IF NOT EXISTS coinbase_transactions(
    id TEXT NOT NULL,
    type TEXT NOT NULL,
    status TEXT NOT NULL,
    amount_amount TEXT NOT NULL,
    amount_currency TEXT NOT NULL,
    native_amount_amount TEXT NOT NULL,
    native_amount_currency TEXT NOT NULL,
    description TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    resource TEXT NOT NULL,
    resource_path TEXT NOT NULL,
    network_status TEXT,
    network_name TEXT,
    to_id TEXT,
    to_resource TEXT,
    to_resource_path TEXT,
    details_title TEXT NOT NULL,
    details_subtitle TEXT NOT NULL,

		PRIMARY KEY (id)
) ;

