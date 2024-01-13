use std::process::exit;

use clap::{Parser, Subcommand, ValueEnum};
use env_logger::Builder;
use simple_client::{error::Result, models::{cache_details::CacheDetails, enums::ticker_symbol::TickerSymbols}, service::ws_client::BinanceWSClient, repository::file_repository::FileRepository};
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
    #[command(long_about = "Retrieve the average price of BTC/USD within given number of seconds")]
    CACHE {
        #[arg(long, short, default_value_t = 10)]
        times: u64
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Args::parse();
    // Builder::from_default_env()
    //     .filter(None, log::LevelFilter::Debug)
    //     .init();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    match cli.mode { 
        Action::CACHE { times } => {

            let cache_details = CacheDetails::new(TickerSymbols::BTCUSDT, times);

            // Send to worker
            let market_data = BinanceWSClient::consume_market_data(&cache_details).await?;
            println!("Cache complete. The average USD price of BTC is {}", market_data.avg_price.expect("Unable to calculate AVG price"));

            // Save to file 
            let _ = FileRepository::save(&market_data)?;
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