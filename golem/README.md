## Golem

Current nodes on mainnet: https://stats.golem.network/ui/

Ports to forward: 3282, 40102, 40103

### Rust

```bash
# one time install
rustup target add wasm32-unknown-emscripten

# compile rust program
cargo rustc --target=wasm32-unknown-emscripten --release -- \
  -C link-args="-s BINARYEN_ASYNC_COMPILATION=0"
```
