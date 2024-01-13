use tokio::time::Duration;

use super::enums::ticker_symbol::TickerSymbols;


#[derive(Debug, Default)]
pub struct CacheDetails { 
    pub symbol: Option<TickerSymbols>,
    pub time_to_record: Duration
}

impl CacheDetails { 
    pub fn new(symbol: TickerSymbols, time_to_record: u64) -> Self { 
        return Self { 
            symbol: Some(symbol), 
            time_to_record: Duration::from_secs(time_to_record)
        };
    }
}