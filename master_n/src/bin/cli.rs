use std::sync::Arc;

use clap::{Parser, Subcommand};
use master_n::{error::Result, models::{cache_details::CacheDetails, enums::ticker_symbol::TickerSymbols}, 
    config::{
        models::{task_queue_message::TaskQueueMessage, MessageType}, 
        message_topics::MessageTopic, kafka_config::kafka_client_config
    }, repository::file_repository::FileRepository};
/*
    simple --mode=cache --times=10
    simple --mode=read
*/
#[derive(Debug, Parser)]
#[command(
    bin_name = "simple",
    name = "simple",
    long_about = "Simple Client for Retrieving Average BTC Prices",
    subcommand_value_name = "mode"
)]
struct CliArgs {
    #[command(subcommand, name = "mode")]
    mode: Action,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Action {
    #[command(long_about = "Read Cache File")]
    READ,
    #[command(long_about = "Retrieve the average price of BTC/USD within given number of seconds")]
    CACHE {
        #[arg(long, short)]
        times: u64
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = CliArgs::parse();
    // Builder::from_default_env()
    //     .filter(None, log::LevelFilter::Debug)
    //     .init();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    match cli.mode { 
        Action::CACHE { times } => {
            // Create a task 
            let cache_details = CacheDetails::new(TickerSymbols::BTCUSDT, times);
            
            // Send a new job to all workers
            let message_topic = MessageTopic::JobTaskQueue;
            let message_type = MessageType::CacheDetails(cache_details);
            let payload = TaskQueueMessage::new(message_topic, message_type);

            // Send to worker
            let _ = kafka_client_config().send_message(&Arc::new(payload)).await?;
        },
        Action::READ => { 

            match FileRepository::load_from_file() {
                Ok(file) => println!("Reading Cache Value: {file:?}"),
                Err(e) => eprintln!("Error Loading from file: {e:?}"),
            }
        }
    }
    Ok(())
}