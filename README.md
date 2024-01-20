# Distributed Price Aggregator Service
**Demo**
https://www.youtube.com/watch?v=JLVSVWbQTUE&t=25s

**Notes
**https://pool-butter-d20.notion.site/SupraOracle-Rust-Client-6cd7b5ffe1f04728bdc341996d1f71e7?pvs=4

The Master Node dispatches tasks via a shared Messaging Queue. Workers subscribe to the queue, process BTC price data from Binance's WebSocket API, and aggregates all price tickers into a single average price. The Master Node aggregates these averages to compute the final average price of the BTC price.


## Client Client (synchronous)
Go to file 

## Simulated Distributed Client (Asynchronous)
To run, you'll need Docker to run all dependencies

Run all workers 
```
    bash run-all-worker-nodes.sh
```
**Run Master Node**
```
    // Start the engine
    cargo run --bin engine

    // Interact with the engine
    cargo run --bin client read 
    cargo run --bin client cache --times=10
```

## Using signatures 
Simple signature checking using K-V checks 
