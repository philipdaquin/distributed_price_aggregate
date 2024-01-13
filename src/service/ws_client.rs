use crate::models::agg_ticker_prices::AggTickerPrices;
use crate::service::aggregate_result::AggTickerPriceService;
use crate::{models::cache_details::CacheDetails, error::ServerError};
use crate::error::Result;
use binance_spot_connector_rust::{
    market::{klines::KlineInterval, self, ticker_price::TickerPrice, avg_price::AvgPrice, agg_trades::AggTrades}, 
    market_stream::{agg_trade::AggTradeStream},
    tokio_tungstenite::BinanceWebSocketClient, 
};
use env_logger::Builder;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, UnboundedSender};
use crate::{models::{enums::ticker_symbol::TickerSymbols, price_ticker::PriceTicker, agg_trade_data::{TradeEvent, BinanceAPIResultWrapper}}};
use futures_util::StreamExt;
use tokio::time::{timeout, Duration};

#[derive(Debug)]
pub struct BinanceWSClient;

impl BinanceWSClient { 
    // Read real time price updates from Binance Websocket API
    #[tracing::instrument(level = "debug")]
    pub async fn read_incoming_price_feed(tx: UnboundedSender<PriceTicker>) -> Result<()> { 
        log::info!("Connecting Client to Binance WebSocket API");
        // 1. Connect to Binance WebSocket Service 
        let (mut conn,  _) = BinanceWebSocketClient::connect_async_default()
            .await
            .expect("Failure to connect to WebSocket Service");
        
        // 2. Listen to real time updates using AggTradeStreams
        let agg_stream = AggTradeStream::new(&TickerSymbols::BTCUSDT.to_string());
        
        // Subscribe to Streams
        log::info!("Listening to Aggregate Trade Streams...");
        conn.subscribe(vec![&agg_stream.into()]).await;
        
        // Consuming messages
        while let Some(message) = conn.as_mut().next().await { 
            match message { 
                Ok(payload) => { 
                    let data = payload.into_data();
                    let trade_event: BinanceAPIResultWrapper = serde_json::from_slice(&data)?;
                    
                    // Added a Default as fallback option
                    let agg_price_data = trade_event.get_agg_data().unwrap_or_default();
                    let price_ticker_data = PriceTicker::from(agg_price_data);
                    
                    // Send to Consumer 
                    let _ = tx.send(price_ticker_data);
                },
                Err(_) => break
            }
        }
        
        conn.close().await.expect("Unable to disconnect");

        return Ok(())

    }

    // Consume real-time price updates using WebSocket Streams
    #[tracing::instrument(level = "debug")]
    pub async fn consume_market_data(cache_details: &CacheDetails) -> Result<AggTickerPrices> { 

        let CacheDetails {symbol, time_to_record} = &cache_details;
        let symbol = symbol.as_ref().expect("Ticker Symbol is Missing");
        
        log::info!("Reading Incoming Market Data For {} ", symbol);
        let (tx, mut rx) = mpsc::unbounded_channel::<PriceTicker>();
        // Start the timeout 
        let task = timeout(cache_details.time_to_record, Self::read_incoming_price_feed(tx)).await;

        // somehow this made the timeout work
        while let Ok(ticker) = task { 
            match ticker {
                Ok(_) => todo!(),
                Err(_) => todo!(),
            }
        }
        // Process Items inside the Consumer 
        let mut agg_price_ticker = AggTickerPriceService::new(symbol.to_owned(), rx);
        let agg_prices = agg_price_ticker.get_agg_ticker_price().await; 
        
        Ok(agg_prices)

    }

}




