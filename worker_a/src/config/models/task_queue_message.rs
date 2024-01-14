use serde_derive::{Serialize, Deserialize};

use crate::{config::message_topics::MessageTopic, models::cache_details::CacheDetails, error::Result};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskQueueMessage { 
    pub message_key: MessageTopic,
    pub job_details: Option<CacheDetails>
}

impl TaskQueueMessage { 
    pub fn new(job_details: CacheDetails) -> Self { 
        return Self { 
            message_key: MessageTopic::AggPriceQueue,
            job_details: Some(job_details)
        }
    }
  
}