use std::{sync::Arc, process::exit};

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use master_n::{error::Result, models::{cache_details::CacheDetails, enums::ticker_symbol::TickerSymbols}, 
    config::{
        models::{task_queue_message::TaskQueueMessage, MessageType}, 
        message_topics::MessageTopic, kafka_config::kafka_client_config
    }, repository::file_repository::FileRepository, server::WorkerServer, client::Client};
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
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    if let Err(e) = run(cli).await { 
        eprintln!("{e}");
        exit(1);
    }

    Ok(())
}



async fn run(operations: CliArgs) -> Result<()> { 
    
    match operations.mode { 
        Action::CACHE { times } => {
            let client = Client::new(TickerSymbols::BTCUSDT, times);
            let _ = client.get_price().await;
            
        },
        Action::READ => {
            let _ = Client::read_local_cache();
            
        }
        
    }
    Ok(())
}