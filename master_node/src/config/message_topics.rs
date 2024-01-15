use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use crate::models::cache_details::CacheDetails;

use super::models::KafkaMessage;

#[derive(Debug, Display, Serialize, Deserialize, EnumString, Clone)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageTopic { 
    JobTaskQueue,
    AggPriceQueue 
}
