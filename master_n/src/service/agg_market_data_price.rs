use tokio::sync::mpsc::UnboundedReceiver;

use crate::models::{price_ticker::PriceTicker, agg_ticker_prices::AggTickerPrices, enums::ticker_symbol::TickerSymbols};


#[derive(Debug)]
pub struct AggMarketDataPriceService { 
    receiver: UnboundedReceiver<AggTickerPrices> 
}

impl AggMarketDataPriceService { 
    pub fn new(receiver: UnboundedReceiver<AggTickerPrices>) -> Self { 
        return Self { 
            receiver
        }
    } 
    #[tracing::instrument(level = "debug")]
    pub async fn get_agg_ticker_price(&mut self) -> AggTickerPrices { 
        let mut queue = Vec::new();
        let mut total_sum = 0.0;
        let mut node_counted = 0;
        let mut global_symbol = TickerSymbols::default();

        while let Some(AggTickerPrices { symbol, raw_data_points, avg_price }) = self.receiver.recv().await { 
            
            if let Some(prices) = avg_price { 
                if prices <= 0.0 { continue }
                total_sum += prices;
                node_counted += 1;
            }
            global_symbol = symbol;
            queue.extend(raw_data_points);
        } 
        
        // The average price of all Average Prices 
        let avg_price = total_sum / node_counted as f64;
        
        let agg_ticker_prices = AggTickerPrices::new(
            global_symbol,
            queue, 
            avg_price
        );

        return agg_ticker_prices
    }
}