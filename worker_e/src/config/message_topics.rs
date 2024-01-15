use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Display, Serialize, Deserialize, EnumString, Copy,  Clone, Eq, PartialEq)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageTopic { 
    JobTaskQueue,
    AggPriceQueue
}
