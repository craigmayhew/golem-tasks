# Golem

This briefly shows you how to install golem, rust, compile golem tasks and deploy to mainnet.

## Setup / install

On windows you may need to enable hyper-v: `hyper-v Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All`

Ports to forward: 3282, 40102, 40103

I also ran into some issues where the golem VM failed, and had to run this `docker-machine rm -f golem` so that Golem pulls a fresh copy and is able to start.

There's a number of other things to install if we want to compile a rust program to wasm.

1. Python (needed for emscripten)
2. emsdk `sudo apt install emscripten`
3. rust (rustup, and then `rustup target add wasm32-unknown-emscripten`)

## Rust

https://docs.golem.network/#/Products/Brass-Beta/gWASM?id=creating-gwasm-tasks-in-golem

### compile rust program
```
cargo rustc --target=wasm32-unknown-emscripten --release -- \
  -C link-args="-s BINARYEN_ASYNC_COMPILATION=0"
```

## Deploy gwasm to main net

1. Send yourself a little glm and a little eth.
2. `golemcli tasks create task.json`
3. `golemcli tasks show`

## Gotchas

1. Every subtask needs a directory within the "in" directory or they timeout.
2. The total timeout needs to be atleast twice the timeout of a single subtask or Golem can't retry them.