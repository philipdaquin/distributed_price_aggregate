use tokio::sync::mpsc::UnboundedReceiver;

use crate::models::{price_ticker::PriceTicker, agg_ticker_prices::AggTickerPrices, enums::ticker_symbol::TickerSymbols};


#[derive(Debug)]
pub struct AggMarketDataPriceService { 
    // receiver: UnboundedReceiver<AggTickerPrices> 
    receiver: Vec<AggTickerPrices>
}

impl AggMarketDataPriceService { 
    pub fn new(receiver: Vec<AggTickerPrices>) -> Self { 
        return Self { 
            receiver
        }
    } 
    #[tracing::instrument(level = "debug")]
    pub async fn get_agg_ticker_price(&mut self) -> AggTickerPrices { 

        log::info!("Calculating sum");
        let mut queue = Vec::new();
        let mut total_sum = 0.0;
        let mut node_counted = 0;
        let mut global_symbol = TickerSymbols::default();
        log::info!("Staritn count");

        for message in &self.receiver { 
            let AggTickerPrices { symbol, raw_data_points, avg_price } = message;
            
            if let Some(prices) = avg_price { 
                log::info!("{prices}");
                if *prices <= 0.0 { continue }
                total_sum += prices;
                node_counted += 1;
            } 
            queue.extend(raw_data_points);         
        } 
        log::info!("emd count");
        
        // The average price of all Average Prices 
        let avg_price = total_sum / node_counted as f64;
        // log::info!("SUM {avg_price}");
        log::info!("Cache complete. The average USD price of BTC is {avg_price}");
        
        let agg_ticker_prices = AggTickerPrices::new(
            global_symbol,
            queue, 
            avg_price
        );

        

        return agg_ticker_prices
    }
}