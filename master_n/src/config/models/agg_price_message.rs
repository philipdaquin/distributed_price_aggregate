use serde_derive::{Deserialize, Serialize};

use crate::models::agg_ticker_prices::AggTickerPrices;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggPriceMessage { 
    node_id: String, 
    node_signature: String, 
    pub agg_ticker_prices: Option<AggTickerPrices>
}


impl AggPriceMessage { 
    pub fn new(agg_ticker_price: AggTickerPrices) -> Self { 
        let node_id = env!("CARGO_PKG_NAME").to_string();
        
        // Temporary Way of signing the signature
        //
        let node_signature = env!("CARGO_PKG_NAME").to_string();
        return Self { 
            node_id, 
            node_signature,
            agg_ticker_prices: Some(agg_ticker_price)
        }
    }
    pub fn get_id(&self) -> String { 
        return self.node_id.to_string();
    }
    pub fn get_signature(&self) -> String { 
        return self.node_signature.to_string();
    }
}