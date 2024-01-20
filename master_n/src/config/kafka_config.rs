use std::sync::{Mutex, Arc};
use std::time::Duration;
use futures::channel::mpsc;
use lazy_static::lazy_static;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::BorrowedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord, DeliveryFuture};
use rdkafka::{ClientConfig, ClientContext, Message};
use once_cell::sync::OnceCell;
use futures_util::StreamExt;
use futures::Future;
use crate::error::Result;
use crate::models::agg_ticker_prices::AggTickerPrices;
use crate::service::worker_service::WorkerService;

use super::message_topics::MessageTopic;
use super::models::task_queue_message::TaskQueueMessage;

lazy_static! {
    static ref KAFKA_BROKER: String = std::env::var("KAFKA_BROKER").unwrap_or("localhost:9092".to_string());
}


pub static KAFKA_CLIENT_CONTEXT: OnceCell<KafkaClientConfig> = OnceCell::new();

#[inline]
pub fn kafka_client_config() -> &'static KafkaClientConfig { 
    KAFKA_CLIENT_CONTEXT.get().expect("Missing Session for Kafka")
}



#[derive(Default, Clone)]
pub struct KafkaClientConfig { 
    pub consumer_config: Option<Arc<StreamConsumer>>, 
    pub producer_config: Option<Arc<FutureProducer>>
}



impl KafkaClientConfig { 

    pub fn new() -> Self { 
        return Self::default();
    }

    pub fn initialise_consumer(&mut self, consumer_topic: MessageTopic) -> &mut Self { 
        let consumer_config: StreamConsumer = ClientConfig::new()
            .set("group.id", "consumer-group")
            .set("bootstrap.servers", KAFKA_BROKER.as_str())
            .set("enable.partition.eof", "true")

            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "latest")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create()
            .expect("Consumer creation failed");
        
        // Listen to Consumer Topics
        let _ = consumer_config.subscribe(&[consumer_topic.to_string().as_str()]);
        let consumer_config = Arc::new(consumer_config);
        
        let _ = self.consumer_config.insert(consumer_config);
        
        return self;
    }

    
    pub fn initialise_producer(&mut self) -> &mut Self { 
        let producer_config: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", KAFKA_BROKER.as_str())
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation failed");
        let producer_config = Arc::new(producer_config);

        let _ = self.producer_config.insert(producer_config);
        return self;
    }

    pub fn build(&mut self) -> Self { 
        // Initialise a singleton pattern for faster access to send messages
        let _ = KAFKA_CLIENT_CONTEXT.set(self.clone());
        return self.clone();
    } 

    pub async fn send_message(&self, message: &Arc<TaskQueueMessage>) -> Result<()> { 
        log::info!("🎉 Sending a message to worker");
        if let Some(ref producer) = &self.producer_config { 
            let binding = message.clone();
            let topic  = &binding.as_ref().message_topic.to_string();
            
            let payload = &serde_json::to_string(binding.as_ref()).unwrap();
            let record = FutureRecord::to(&topic)
                .payload(payload)
                .key(&topic);

            log::info!("📦 Sending payload to Aggregated Price Message Payload");
            let _ = producer.send(record, Duration::from_secs(10)).await;
            
            log::info!("🎉 Future completed. Result");
        }       
         

        Ok(())
    }

    pub async fn consume_messages(&self) -> Result<()> { 
        // let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<AggTickerPrices>();
        let mut queue = Vec::new();
        if let Some(ref consumer) = &self.consumer_config { 
            loop { 
                match consumer.recv().await {
                    //
                    //  Collect all `AggTickerPrices` in a single in-memory queue, then
                    //  values have been collected, run the process of aggregating into a single value
                    //
                    Ok(payload) => {
                        let payload = MessagePayload::from(&payload);
                        let task_details: TaskQueueMessage = serde_json::from_str(payload.as_str())?;
                        log::info!("👷 Send over to be process by workers");
                        // Validate 
                        if let Some(ticker_price) = WorkerService::validate_message_type(task_details) { 
                            queue.push(ticker_price);
                        } else { 
                            log::warn!("Ignoring invalid messages from unverified nodes...");
                            continue;
                        }
                    },
                    Err(_) => {
                        log::info!("Length of QUEUE: {}", queue.len());

                        if !queue.is_empty() {
                            if queue.len() == 5 { 
                                // proess the message with retries  
                                if let Err(e) = WorkerService::process_worker_task_with_retries(&queue).await { 
                                    eprintln!("Server Error: {e}");
                                }
                            } else if queue.len() > 0 { 
                                log::warn!("Unable to retrieve all values from all nodes...");
                                log::warn!("Calculating the average anyway with {} nodes", queue.len());
                                // proess the message with retries  
                                if let Err(e) = WorkerService::process_worker_task_with_retries(&queue).await { 
                                    eprintln!("Server Error: {e}");
                                }
                            } else { 
                                queue.clear();
                            }
                        } else { 
                            log::warn!("Waiting for new tasks...");
                            continue;
                        }

                    },
                }
            }
        }

        Ok(())
    }
    
}


#[derive(Debug, Clone)]
pub struct MessagePayload(String);

impl MessagePayload {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// generic way to turn a borrowed message into a (wrapped) string
impl<'a> From<&'a BorrowedMessage<'a>> for MessagePayload {
    fn from(bm: &'a BorrowedMessage) -> Self {
        match bm.payload_view::<str>() {
            Some(Ok(s)) => MessagePayload(String::from(s)),
            Some(Err(e)) => MessagePayload(format!("{:?}", e)),
            None => MessagePayload(String::from("")),
        }
    }
}
