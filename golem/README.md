## Golem

Current nodes on mainnet: https://stats.golem.network/ui/

Ports to forward: 3282, 40102, 40103

There's a number of other things to install if we want to compile a rust program to wasm.

1. Python (needed for emscripten)
2. emsdk (see below for specific instructions)
3. rust (rustup, and then a wasm target)

## emsdk

```bash
git clone https://github.com/emscripten-core/emsdk.git
cd emsdk
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh
```

### Rust

https://docs.golem.network/#/Products/Brass-Beta/gWASM?id=creating-gwasm-tasks-in-golem

```bash
# one time install
rustup target add wasm32-unknown-emscripten

# compile rust program
cargo rustc --target=wasm32-unknown-emscripten --release -- \
  -C link-args="-s BINARYEN_ASYNC_COMPILATION=0"
```

### Deploy gwasm to main net

1. Send yourself a little glm and a little eth.
2. `golemcli tasks create task.json`
3. `golemcli tasks show`
