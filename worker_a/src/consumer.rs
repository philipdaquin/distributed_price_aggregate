use crate::config::message_topics::MessageTopic;


#[derive(Debug, Clone)]
pub struct GenericKafkaConsumer { 
    pub topic: MessageTopic
}

impl GenericKafkaConsumer { 
    pub async fn run_consumer_client() { 
        
    }
}