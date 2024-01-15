use serde_derive::{Serialize, Deserialize};

use crate::{config::message_topics::MessageTopic, models::cache_details::CacheDetails, error::Result};

use super::MessageType;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskQueueMessage { 
    pub message_topic: MessageTopic,
    pub message_type: Option<MessageType>
}

impl TaskQueueMessage { 
    pub fn new(message_topic: MessageTopic, message_type: MessageType) -> Self { 
        return Self { 
            message_topic,
            message_type: Some(message_type)
        }
    }
  
}