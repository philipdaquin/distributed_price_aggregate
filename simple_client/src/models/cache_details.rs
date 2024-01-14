use tokio::time::Duration;

use super::enums::ticker_symbol::TickerSymbols;


#[derive(Debug)]
pub struct CacheDetails { 
    pub symbol: Option<TickerSymbols>,
    pub time_to_record: Duration
}

impl Default for CacheDetails { 
    fn default() -> Self {
        return Self { 
            symbol: Some(TickerSymbols::BTCUSDT),
            time_to_record: Duration::from_secs(10)
        }
    }
}

impl CacheDetails { 
    pub fn new(symbol: TickerSymbols, time_to_record: u64) -> Self { 
        return Self { 
            symbol: Some(symbol), 
            time_to_record: Duration::from_secs(time_to_record)
        };
    }
}