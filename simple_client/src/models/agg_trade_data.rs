use serde::{Deserializer, Deserialize as SerdeDeserializer, self};
use serde_derive::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use super::{enums::ticker_symbol::TickerSymbols, price_ticker::PriceTicker};
use crate::error::{Result, ServerError};
use serde_this_or_that::as_f64;


#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct BinanceAPIResultWrapper {
    #[serde(flatten)]
    pub result: Option<TradeEvent>,
    pub id: Option<i64>, 
}



#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct TradeEvent { 
    pub stream: String, 
    pub data: Option<AggTradeData>
}


#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct AggTradeData { 
    // Event Type 
    #[serde(rename = "e")]
    event: String,
    
    // Event time
    #[serde(rename = "E")]
    event_time: i64,
    
    // Symbol
    #[serde(rename = "s")]
    pub symbol: TickerSymbols,
    
    // Aggregate trade ID
    #[serde(rename = "a")]
    agg_trade_id: i64, 
    
    // Price
    #[serde(rename = "p", deserialize_with = "as_f64")]
    pub price: f64,
    
    // Quantity
    #[serde(rename = "q")]
    quantity: String, 
    
    // First Trade ID
    #[serde(rename = "f")]
    first_trade_id: u64, 
    
    // Last Trade ID
    #[serde(rename = "l")]
    last_trade_id: u64, 
    
    // Trade Time
    #[serde(rename = "T")]
    trade_time: u64,
    
    // Is the buyer the market maker
    #[serde(rename = "m")]
    is_mm: bool, 
    // Ignore
    #[serde(rename = "M")]
    ignore: bool
}




impl BinanceAPIResultWrapper { 
    pub fn get_agg_data(&self) -> Option<AggTradeData> { 
        return self.result.as_ref()?.data.clone()
    }
}


impl From<AggTradeData> for PriceTicker {
    fn from(value: AggTradeData) -> Self {
        return PriceTicker::new(value.symbol, value.price);
    }
}

#[cfg(test)]
mod ticker_stream_unit_test { 
    use super::*;

    const sample_data: &str = r#"
        {
            "stream": "btcusdt@aggTrade",
            "data": {
                "e": "aggTrade",
                "E": 1705112466818,
                "s": "BTCUSDT",
                "a": 2828299942,
                "p": "42793.82000000",
                "q": "0.02439000",
                "f": 3370867322,
                "l": 3370867322,
                "T": 1705112466717,
                "m": true,
                "M": true
            }
        }
    "#;

    fn get_agg_data() -> Result<AggTradeData> { 
        let trade_event: TradeEvent = serde_json::from_str(sample_data)?;
        let agg_trade_data: AggTradeData = trade_event.data.unwrap_or_default();  
        return Ok(agg_trade_data);
    }

    #[test]
    fn test_deserialisation() -> Result<()> { 
        // Convert to TradeEvent data type 
        let trade_event: TradeEvent = serde_json::from_str(sample_data)?;
        let agg_trade_data: AggTradeData = trade_event.data.unwrap_or_default();        
        println!("{agg_trade_data:#?}");

        assert_eq!(agg_trade_data.event,                    String::from("aggTrade"));
        assert_eq!(agg_trade_data.event_time,               1705112466818);
        assert_eq!(agg_trade_data.symbol,                   TickerSymbols::BTCUSDT);
        assert_eq!(agg_trade_data.agg_trade_id,             2828299942);
        assert_eq!(agg_trade_data.price,                    42793.82);
        assert_eq!(agg_trade_data.quantity,                 String::from("0.02439000"));
        assert_eq!(agg_trade_data.first_trade_id,           3370867322);
        assert_eq!(agg_trade_data.last_trade_id,            3370867322);
        assert_eq!(agg_trade_data.trade_time,               1705112466717);
        assert_eq!(agg_trade_data.is_mm,                    true);
        assert_eq!(agg_trade_data.ignore,                   true);


        Ok(())
    }
    #[test]
    fn test_decontruct_aggdata_to_price_ticker() { 
        let agg_data = get_agg_data().unwrap();
        let cloned_agg_data = agg_data.clone();


        let  price_ticker  = PriceTicker::from(agg_data);
        println!("{price_ticker:#?}");
        
        let PriceTicker { symbol, avg_price } = price_ticker;
        
        assert_eq!(symbol.unwrap(), cloned_agg_data.symbol);
        assert_eq!(avg_price, cloned_agg_data.price);
    }

}
