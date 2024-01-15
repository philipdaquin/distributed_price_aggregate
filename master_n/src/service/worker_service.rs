use std::sync::Arc;

use tokio::sync::mpsc;

use crate::config::kafka_config::{KafkaClientConfig, kafka_client_config};
use crate::config::message_topics::MessageTopic;
use crate::config::models::MessageType;
use crate::config::models::agg_price_message::AggPriceMessage;
use crate::config::models::task_queue_message::TaskQueueMessage;
use crate::error::Result;
use crate::models::agg_ticker_prices::AggTickerPrices;
use crate::models::cache_details::CacheDetails;
use crate::models::price_ticker::PriceTicker;
use crate::repository::file_repository::FileRepository;
use crate::service::validate_service::initialise_valid_signatures;

use super::agg_market_data_price::AggMarketDataPriceService;

#[derive(Debug)]
pub struct WorkerService;

impl WorkerService { 
    /**
     *  Consume AggregatedPriceData from the worker nodes
     * 
    */
    #[tracing::instrument(level = "debug", err)]
    pub async fn send_jobs(cache_details: CacheDetails) -> Result<()> { 
        // Create a task 
        let message_topic = MessageTopic::JobTaskQueue;
        let message_type = MessageType::CacheDetails(cache_details);
        let payload = TaskQueueMessage::new(message_topic, message_type);

        // Send to worker
        log::info!("Sending new job to workers");
        let _ = kafka_client_config().send_message(&Arc::new(payload)).await?;

        Ok(())
    }
    #[tracing::instrument(level = "debug", err)]
    pub async fn process_worker_task(task: TaskQueueMessage) -> Result<()> { 

        let validator = initialise_valid_signatures();

        let (tx, rx) = mpsc::unbounded_channel::<AggTickerPrices>();
        let mut tmp: Vec<AggTickerPrices> = Vec::new();
        log::info!("📘 Processing Data Prices");
        if let Some(job) = task.message_type { 
            if let MessageType::AggPriceMessage(agg_message) = job { 
                // Validate node signatures 
                log::info!("🎉 Validating Node Signature");
                if validator.validate_sig(agg_message.get_id(), agg_message.get_signature()) { 
                    if let Some(data) = agg_message.agg_ticker_prices { 
                        // let _ = tx.send(data);
                        tmp.push(data)
                    }
                } 
            }
        }

        // Process all AggTickerPrices inside the Channel
        log::info!("Calculating final market data");
        let mut agg_price_ticker = AggMarketDataPriceService::new(tmp);
        let market_data = agg_price_ticker.get_agg_ticker_price().await; 
        log::info!("Cache complete. The average USD price of BTC is {}", market_data.avg_price.expect("Unable to calculate AVG price"));
        
        // Save locally to database
        log::info!("Saving locally");
        let _ = FileRepository::save(&market_data)?;     

        Ok(())
    }
}