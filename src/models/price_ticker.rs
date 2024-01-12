use serde_derive::Deserialize;

use super::enums::ticker_symbol::TickerSymbols;

#[derive(Debug, Deserialize)]
pub struct PriceTicker { 
    symbol: Option<TickerSymbols>,
    avg_price: f64
}