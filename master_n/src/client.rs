
use std::sync::Arc;

use crate::{error::Result, models::{cache_details::CacheDetails, enums::ticker_symbol::TickerSymbols}, config::{message_topics::MessageTopic, models::{task_queue_message::TaskQueueMessage, MessageType}, kafka_config::kafka_client_config}, repository::file_repository::FileRepository};


#[derive(Clone, Debug)]
pub struct Client { 
    cache_details: CacheDetails
}

impl Client { 

    pub fn new(symbol: TickerSymbols, times: u64) -> Self { 
        let cache_details = CacheDetails::new(symbol, times);
        return Self  {
            cache_details
        }
    }

    pub fn connect() -> Result<()> { 
        todo!()
    }

    pub async fn get_price(&self) -> Result<()> { 
        let server = "http://localhost:4000/api/send";
        // Create a task 
        let json_body = serde_json::to_string(&self.cache_details).expect("Unable to serialize");


        let client = reqwest::Client::new();
        let response = client
            .post(server)
            .header("Content-Type", "application/json")
            .body(json_body)
            .send()
            .await
            .expect("Unexpected error on Reqwest!");

        Ok(())
    }  

    pub fn read_local_cache() -> Result<()> { 
        match FileRepository::load_from_file() {
            Ok(file) => println!("Reading Cache Value: {file:#?}"),
            Err(e) => eprintln!("Error Loading from file: {e:?}"),
        }

        Ok(())
    }
}