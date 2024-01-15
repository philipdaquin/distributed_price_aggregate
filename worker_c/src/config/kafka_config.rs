use std::sync::{Mutex, Arc};
use std::time::Duration;
use futures::Future;
use lazy_static::lazy_static;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord, DeliveryFuture};
use rdkafka::util::Timeout;
use rdkafka::{ClientConfig, ClientContext, Message};
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;

use crate::error::Result;
use crate::models::cache_details::CacheDetails;
use crate::service::worker_service::WorkerService;

use super::message_topics::MessageTopic;
use super::models::MessageType;
use super::models::agg_price_message::AggPriceMessage;
use super::models::task_queue_message::TaskQueueMessage;

lazy_static! {
    static ref KAFKA_BROKER: String = std::env::var("KAFKA_BROKER").expect("Can't read Kafka broker address");
}


pub static KAFKA_CLIENT_CONTEXT: OnceCell<KafkaClientConfig> = OnceCell::new();

#[inline]
pub(crate) fn kafka_client_config() -> &'static KafkaClientConfig { 
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
            .set("group.id", "consumer-grou-3")
            .set("bootstrap.servers", KAFKA_BROKER.as_str())
            .set("session.timeout.ms", "6000")
            .set("enable.partition.eof", "true")

            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", "earliest")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create()
            .expect("Consumer creation failed");
        
        // Listen to Consumer Topics
        let _ = consumer_config.subscribe(&[consumer_topic.to_string().as_str()]).unwrap();
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
        if let Some(ref producer) = &self.producer_config { 

            let binding = message.clone();
            let topic  = &binding.as_ref().message_topic.to_string();
            
            let payload = &serde_json::to_string(binding.as_ref()).unwrap();
            let record = FutureRecord::to(&topic)
                .payload(payload)
                .key(&topic);

            log::info!("🚀 Sending payload to Aggregated Price Message Payload");
            let _ = producer.send(record, Timeout::After(Duration::from_secs(0))).await;
                
        }        

        Ok(())
    }

    pub async fn consume_messages(&self) -> Result<()> { 
        if let Some(ref consumer) = &self.consumer_config { 
            loop { 
                match consumer.recv().await {
                    Ok(payload) => {
                        let payload = MessagePayload::from(&payload);
                        let task_details: TaskQueueMessage = serde_json::from_str(payload.as_str())?;

                        log::info!("Send over to be process by workers");
                        let _ = WorkerService::process_worker_task(task_details).await?;
                    },
                    Err(_) => log::error!(""),
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
