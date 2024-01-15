use std::sync::Arc;

use crate::config::kafka_config::{KafkaClientConfig, kafka_client_config};
use crate::config::message_topics::MessageTopic;
use crate::config::models::MessageType;
use crate::config::models::agg_price_message::AggPriceMessage;
use crate::config::models::task_queue_message::TaskQueueMessage;
use crate::error::Result;
use crate::models::cache_details::CacheDetails;
use crate::repository::file_repository::FileRepository;
use crate::service::ws_client::BinanceWSClient;

#[derive(Debug)]
pub struct WorkerService;

impl WorkerService { 
    #[tracing::instrument(level = "debug", err)]
    pub async fn process_worker_task(task: TaskQueueMessage) -> Result<()> { 
        
        if let Some(job) = task.message_type { 
            if let MessageType::CacheDetails(message) = &job { 
                // Send to worker
                let market_data = BinanceWSClient::consume_market_data(&message).await?;
                println!("Cache complete. The average USD price of BTC is {}", market_data.avg_price.expect("Unable to calculate AVG price"));
                let agg_market_message = AggPriceMessage::new(market_data);
                
                // Save to file 
                // log::info!("Saving Aggregated Prices locally");
                // let _ = FileRepository::save(&market_data)?;
                // Send Aggregated Price Data back to the Master Node 
                log::info!("Sending Aggregate Market Data to the Master Node.");
                let message_type = MessageType::AggPriceMessage(agg_market_message);
                let payload = TaskQueueMessage::new(task.message_topic, message_type);

                let _ = kafka_client_config().send_message(&Arc::new(payload)).await?;
            }
        }
        
        Ok(())
    }
}