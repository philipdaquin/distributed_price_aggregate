use super::{price_ticker::PriceTicker, enums::ticker_symbol::TickerSymbols};

#[derive(Clone, Debug)]
pub struct AggTickerPrices { 
    pub symbol: TickerSymbols,
    pub raw_data_points: Vec<PriceTicker>,
    pub avg_price: Option<f64>
}

impl AggTickerPrices { 
    pub fn new(
        symbol: TickerSymbols, 
        raw_data_points: Vec<PriceTicker>, 
        avg_price: f64
    ) -> Self { 
        return Self { 
            symbol, raw_data_points, avg_price: Some(avg_price)
        }
    }
}