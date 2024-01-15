# Distributed Price Aggregator Service

## Simple Client
Go to file 

## Simulated Distributed Client 
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