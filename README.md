
# Crypto Portfolio Performance Bridge

This project is a command line tool to generate CSV of crypto trades into portfolio performance format.


## Data structure

### Actions done on Web3

#### CEX Actions (e.g. Mexc)
- Deposit fiat
- Buy crypto
- Sell crypto
- Withdraw fiat
- Withdraw crypto 
- Swap crypto 

#### DEX Actions
- Deposit crypto on a wallet
- Swap crypto
- Withdraw crypto from a wallet
- bridge between networks



## "Probleme" mit Portfolio Performance

- "Umbuchungen" haben keine Transaktionsgebühren
- Dexscrenner für Kurse, eventuell einen anderen anbieter?
- "Airdrops" nicht direkt möglich, weil 0,00$ als Kaufpreis nicht möglich ist
- Crypto als "Währung"?
- Direkte Anbindung von Mexc, Bridges, etc. in PP? Oder lieber ein separates Tool zum Importieren?
