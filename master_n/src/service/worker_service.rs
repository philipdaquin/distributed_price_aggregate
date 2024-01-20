use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::sleep;

use crate::config::kafka_config::{KafkaClientConfig, kafka_client_config};
use crate::config::message_topics::MessageTopic;
use crate::config::models::MessageType;
use crate::config::models::agg_price_message::AggPriceMessage;
use crate::config::models::task_queue_message::TaskQueueMessage;
use crate::error::{Result, ServerError};
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

    #[tracing::instrument(level = "debug")]
    pub fn validate_message_type(task: TaskQueueMessage) -> Option<AggTickerPrices> { 
        let validator = initialise_valid_signatures();

        log::info!("📘 Processing Data Prices");
        if let Some(job) = task.message_type { 
            if let MessageType::AggPriceMessage(agg_message) = job { 
                // Validate node signatures 
                let id = agg_message.get_id();
                log::info!("🎉 Validating Node Signature of worker: {id}");
                if validator.validate_sig(id.to_string(), agg_message.get_signature()) { 
                    if let Some(data) = agg_message.agg_ticker_prices { 
                        return Some(data);
                    }
                } 
            }
        }
        return None;
    }

    #[tracing::instrument(level = "debug", err)]
    pub async fn process_worker_task_with_retries(task: &Vec<AggTickerPrices>) -> Result<()> { 
        const MAX_RETRIES: usize = 3;
        let mut retries =  3;
        let mut agg_price_ticker = AggMarketDataPriceService::new(task.to_vec());
        loop { 
            match agg_price_ticker.get_agg_ticker_price().await {
                Ok(_) => {
                    log::info!("🧮 Calculating final market data");
                    let market_data = agg_price_ticker.get_agg_ticker_price().await?; 
                    log::info!("✏️ Cache complete. The average USD price of BTC is {}", market_data.avg_price.expect("Unable to calculate AVG price"));
                    
                    // Save locally to database
                    log::info!("💾 Saving locally");
                    let _ = FileRepository::save(&market_data)?;  
                    return Ok(());
                },
                Err(e) => { 
                    log::error!("Error processing messages, retrying ({}): {:?}", retries + 1, e);
                    retries += 1;
                    if retries >= MAX_RETRIES { 
                        return Err(ServerError::HttpClientError);
                    }

                    sleep(Duration::from_secs(1)).await;

                },
            }
        }
    }
}