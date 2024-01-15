
# Master Node
The Master Node
1. Create a new job schedule for all workers
2. Waits for the messages to arrive
3. Master Node validates the signature of each messages
4. Then, calculates the AVG price of BTC / USDT 

### Ensure all dependencies are running first including the Master Node
```

    bash local-run-deps

```

### Cache the real time aggregated price tickers under a given time limit
```

    cargo run -- cache --times=10 

```

### Read the local cache file 
```

    cargo run -- read 

```