use actix_cors::Cors;
use actix_web::{get, middleware::Logger, route, web, App, HttpServer, Responder};

use crate::config::{kafka_config::KafkaClientConfig, message_topics::MessageTopic};




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
        
        // Server attributes
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let address = format!("{}:{}", host, self.port);
    
        log::info!("Starting {} Node: {}", env!("CARGO_PKG_NAME"), self.port);

        // Initialise Kafka Consumer and Producers
        let kafka_manager = KafkaClientConfig::new()
            .initialise_consumer(MessageTopic::JobTaskQueue)
            .initialise_producer()
            .build();

        let consumer = tokio::spawn(async move { 
            let _ = kafka_manager.consume_messages().await;        
        });

        tokio::select! {
            _ = consumer => {}
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

