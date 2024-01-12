use super::enums::ticker_symbol::TickerSymbols;


#[derive(Debug, Default)]
pub struct CacheDetails { 
    symbol: Option<TickerSymbols>,
    time_to_record: i32
}

impl CacheDetails { 
    pub fn new(symbol: TickerSymbols, time_to_record: i32) -> Self { 
        return Self { 
            symbol: Some(symbol), 
            time_to_record
        };
    }
}