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

use super::agg_market_data_price::AggMarketDataPriceService;

#[derive(Debug)]
pub struct WorkerService;

impl WorkerService { 
    /**
     *  Consume AggregatedPriceData from the worker nodes
     * 
     */
    #[tracing::instrument(level = "debug", err)]
    pub async fn process_worker_task(task: TaskQueueMessage) -> Result<AggTickerPrices> { 
        let (tx, rx) = mpsc::unbounded_channel::<AggTickerPrices>();

        if let Some(job) = task.message_type { 
            if let MessageType::AggPriceMessage(AggPriceMessage { agg_ticker_prices, .. }) = job { 

                // Validate node signatures 
                //
                //

                // Aggregate all AggTickerPrices into a single channel
                if let Some(ticker_prices) = agg_ticker_prices { 
                    let _ = tx.send(ticker_prices);
                }
            }
        }

        // Process all AggTickerPrices inside the Channel
        let mut agg_price_ticker = AggMarketDataPriceService::new(rx);
        let agg_prices = agg_price_ticker.get_agg_ticker_price().await; 
        

        log::info!("{agg_prices:?}");

        Ok(agg_prices)
    }
}