## Golem

Current nodes on mainnet: https://stats.golem.network/ui/

Ports to forward: 3282, 40102, 40103

## emsdk

```bash
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh
```

### Rust

```bash
# one time install
rustup target add wasm32-unknown-emscripten

# compile rust program
cargo rustc --target=wasm32-unknown-emscripten --release -- \
  -C link-args="-s BINARYEN_ASYNC_COMPILATION=0"
```
