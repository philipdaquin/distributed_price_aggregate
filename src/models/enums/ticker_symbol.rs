use serde_derive::Deserialize;
use strum_macros::EnumString;

#[derive(Debug, Default, Deserialize, EnumString)]
pub enum TickerSymbols { 
    #[default]
    BTCUSD,
    BNBBTC
}

