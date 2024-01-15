use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder};
use tokio::sync::oneshot;
use crate::{config::{kafka_config::KafkaClientConfig, message_topics::MessageTopic}, error::ServerError, models::{cache_details::CacheDetails, agg_ticker_prices::AggTickerPrices}, repository::file_repository::FileRepository, controller::configure_analysis_service};

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
        
        // let (resp_tx, resp_rx) = oneshot::channel::<AggTickerPrices>();
        // Server attributes
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let address = format!("{}:{}", host, self.port);
    
        log::info!("Starting {} Node: {}", env!("CARGO_PKG_NAME"), self.port);

        // Initialise Kafka Consumer and Producers
        let kafka_manager = KafkaClientConfig::new()
            .initialise_consumer(MessageTopic::AggPriceQueue)
            .initialise_producer()
            .build();

        // Kafka Manager 
        let kafka_manager = Arc::new(kafka_manager);
        let binding = kafka_manager.clone();


        tokio::select! { 
            _ = async { 
                    let _ = kafka_manager.clone().consume_messages().await; 
            } => {}
            _ = async {
                let da = web::Data::new(binding.clone());
                HttpServer::new(move || {
                    App::new()
                        .app_data(da.clone())
                        .wrap(Cors::permissive())
                        .wrap(Logger::default())
                        .configure(configure_analysis_service)
                })
                .workers(2)
                .bind(address)
                .unwrap()
                .run()
                .await
                .unwrap();
            } => {}
        }


        Ok(())
    }
}

