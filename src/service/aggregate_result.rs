use tokio::sync::mpsc::UnboundedReceiver;

use crate::models::{price_ticker::PriceTicker, agg_ticker_prices::AggTickerPrices, enums::ticker_symbol::TickerSymbols};


#[derive(Debug)]
pub struct AggTickerPriceService { 
    symbol: TickerSymbols,
    receiver: UnboundedReceiver<PriceTicker> 
}

impl AggTickerPriceService { 
    pub fn new(symbol: TickerSymbols, receiver: UnboundedReceiver<PriceTicker>) -> Self { 
        return Self { 
            symbol, receiver
        }
    } 
    
    pub async fn get_agg_ticker_price(&mut self) -> AggTickerPrices { 
        let symbol = self.symbol.clone();
        let mut queue = Vec::new();
        let mut total_sum = 0.0;

        while let Some(tickers) = self.receiver.recv().await { 
            if tickers.avg_price <= 0.0 { continue }
            total_sum += tickers.avg_price;
            queue.push(tickers);
        } 
        
        let avg_price = total_sum / queue.len() as f64;
        
        let agg_ticker_prices = AggTickerPrices::new(
            symbol,
            queue, 
            avg_price
        );

        return agg_ticker_prices
    }
}