use serde_derive::{Deserialize, Serialize};

use super::enums::ticker_symbol::TickerSymbols;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PriceTicker { 
    pub symbol: Option<TickerSymbols>,
    pub avg_price: f64
}

impl PriceTicker { 
    pub fn new(symbol: TickerSymbols, avg_price: f64) -> Self { 
        return Self { 
            symbol: Some(symbol), avg_price
        }
    }
}