use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder};
use tokio::sync::oneshot;

use crate::{config::{kafka_config::KafkaClientConfig, message_topics::MessageTopic}, error::ServerError, models::{cache_details::CacheDetails, agg_ticker_prices::AggTickerPrices}, repository::file_repository::FileRepository};




#[derive(Debug)]
pub struct WorkerServer { 
    port: u32
}

impl WorkerServer { 
    pub fn new(port: u32) -> Self { 
        return Self { 
            port
        }
    }

    #[tracing::instrument(level = "debug", err)]
    pub async fn run_server(&self) -> std::io::Result<()> {
        

        let (resp_tx, resp_rx) = oneshot::channel::<AggTickerPrices>();


        // Server attributes
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let address = format!("{}:{}", host, self.port);
    
        log::info!("Starting {} Node: {}", env!("CARGO_PKG_NAME"), self.port);

        // Initialise Kafka Consumer and Producers
        let kafka_manager = KafkaClientConfig::new()
            .initialise_consumer(MessageTopic::AggPriceQueue)
            .initialise_producer()
            .build();

        let consumer = tokio::spawn(async move { 
            let agg = kafka_manager.consume_messages().await.expect("");       
            if let Some(prices) = agg { 
                resp_tx.send(prices).expect("");
            }
        });

        // Ideally, we are listening to both the consumer and system inputs 
        tokio::select! {
            _ = consumer => {
                let market_data: AggTickerPrices = resp_rx.await.expect("");

                println!("Cache complete. The average USD price of BTC is {}", 
                    market_data.avg_price.expect("Unable to calculate AVG price"));

                // save to local database
                let _ = FileRepository::save(&market_data);
            }
        }
        
        HttpServer::new(move || {
            App::new()
                .wrap(Cors::permissive())
                .wrap(Logger::default())
        })
        .workers(2)
        .bind(address)?
        .run()
        .await
    }
}

