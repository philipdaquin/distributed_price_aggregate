use std::fmt::Debug;

use serde::{Serialize, de::DeserializeOwned};
use serde_derive::Deserialize;

use crate::models::cache_details::CacheDetails;

use self::agg_price_message::AggPriceMessage;

pub mod agg_price_message;
pub mod task_queue_message;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum KafkaMessage { 
    CacheDetails(CacheDetails),
    AggPriceMessage(AggPriceMessage)
}
