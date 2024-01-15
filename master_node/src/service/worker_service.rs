use std::fmt::Debug;

use crate::config::kafka_config::{KafkaClientConfig, kafka_client_config};
use crate::config::message_topics::MessageTopic;
use crate::config::models::KafkaMessage;
use crate::config::models::task_queue_message::TaskQueueMessage;
use crate::error::Result;

use super::aggregate_result::AggTickerPriceService;


#[derive(Debug)]
pub struct WorkerService;

impl WorkerService { 
    /**
     * 
     * Master Node receives the AggTickerPrices
     * 
     */
    #[tracing::instrument(level = "debug", err)]
    pub async fn process_worker_task(task: TaskQueueMessage) -> Result<()> { 
       
        if let Some(job) = task.job_details { 
            if let KafkaMessage::AggPriceMessage(message) = job { 

                
            }
        }
        
        Ok(())
    }
}