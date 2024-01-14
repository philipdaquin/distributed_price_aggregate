use std::sync::Arc;

use crate::config::kafka_config::{KafkaClientConfig, kafka_client_config};
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
    pub async fn process_worker_task(job_details: TaskQueueMessage) -> Result<()> { 
        
        if let Some(job) = job_details.job_details { 
            // Send to worker
            let market_data = BinanceWSClient::consume_market_data(&job).await?;
            println!("Cache complete. The average USD price of BTC is {}", market_data.avg_price.expect("Unable to calculate AVG price"));
            
            let agg_market_message = AggPriceMessage::new(market_data);
            // Save to file 
            // log::info!("Saving Aggregated Prices locally");
            // let _ = FileRepository::save(&market_data)?;
            
            log::info!("Sending Aggregate Market Data");
            let _ = kafka_client_config()
                .send_message(&Arc::new(agg_market_message), &Arc::new(job_details.message_key))
                .await?;
        }
        
        Ok(())
    }
}