use serde_derive::Deserialize;
use strum_macros::{Display, EnumString};

#[derive(Debug, Default, Display, Deserialize, EnumString, Clone, Eq, PartialEq)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TickerSymbols { 
    #[default]
    BTCUSDT,
    BNBBTC
}

