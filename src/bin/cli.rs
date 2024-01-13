use std::process::exit;

use clap::{Parser, Subcommand, ValueEnum};
use env_logger::Builder;
use simple_client::{error::Result, models::{cache_details::CacheDetails, enums::ticker_symbol::TickerSymbols}, service::ws_client::BinanceWSClient};
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
struct Args {
    #[command(subcommand, name = "mode")]
    mode: Action,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Action {
    #[command(long_about = "Read Cache File")]
    READ,
    #[command(long_about = "Retrieve the average price of BTC/USD with given number of seconds")]
    CACHE {
        #[arg(long)]
        times: u64
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Args::parse();
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();
    match cli.mode { 
        Action::CACHE { times } => {

            let cache_details = CacheDetails::new(TickerSymbols::BTCUSDT, times);

            // Send to worker
            let _ = BinanceWSClient::consume_market_data(&cache_details).await?;
            

            println!("Monitoring BTC under {times} seconds ");
        },
        Action::READ => { 
            println!("Reading BTC price");

        }
    }
    Ok(())
}