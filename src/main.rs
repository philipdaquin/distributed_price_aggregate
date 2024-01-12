use std::{thread, time::Duration};

use binance_spot_connector_rust::{
    market::{klines::KlineInterval, self}, market_stream::ticker::TickerStream,
    tokio_tungstenite::BinanceWebSocketClient, 
    hyper::BinanceHttpClient, 
};
use env_logger::Builder;
const BINANCE_WSS_BASE_URL: &str = "wss://stream.binance.com:9443/ws";

use simple_client::{error::Result, models::enums::ticker_symbol::TickerSymbols};

use std::str::FromStr;
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> { 
    Builder::from_default_env()
        .filter(None, log::LevelFilter::Debug)
        .init();
    // Establish connection
    // let (mut conn,  response) = BinanceWebSocketClient::connect_async(BINANCE_WSS_BASE_URL)
    //     .await
    //     .expect("Failed to connect");
    // log::info!("{:?}", response);
    
    let (mut conn,  response) = BinanceWebSocketClient::connect_async_default()
    .await
    .expect("Failed to connect");


    // Subscribe to streams
    conn.subscribe(vec![&TickerStream::from_symbol("BTCUSDT").into()]).await;

    while let Some(message) = conn.as_mut().next().await { 
        match message { 
            Ok(payload) => { 
                let data = payload.into_data();
                let string_data = String::from_utf8(data).expect("");
                log::info!("{}", &string_data);
            },
            Err(_) => break
        }
    }
    // thread::sleep(Duration::from_secs(10))
    // // Disconnect
    conn.close().await.expect("Failed to disconnect");

    // let ticker = TickerSymbols::BTCUSDT.to_string();
    // let client = BinanceHttpClient::default();
    // let request = market::ticker_price().symbol(ticker.as_str());

    // let data = client.send(request)
    //     .await?
    //     .into_body_str()
    //     .await?;

    // log::info!("{:?}", data);

    Ok(())

}