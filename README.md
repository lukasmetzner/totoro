# Totoro
## Simple Message Queue

- One instance has exactly one queue
    - No multi topic functionality
- Max messages in queue: 64000

## Run
```
cargo run --bin server
cargo run --bin sub-client
cargo run --bin pub-client
```