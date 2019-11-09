# Compression Task

This is using a basic (possibly not very effective due the the rust crate we are using) lzma compression alogrithm. `in.txt` is turned into `in.txt.7z`.

cd to "in" directory and compile:
```bash
cargo rustc --target=wasm32-unknown-emscripten --release -- -C link-args="-s BINARYEN_ASYNC_COMPILATION=0" --verbose && cp ../target/wasm32-unknown-emscripten/release/compression* ./
```

## Performance
WARNING - All files that we hope to compress cause upload bandwidth to be many times the original file size. This could relate to each subtask being sent to multiple nodes for verification. Task not yet run on a 1MB+ file, last tried on golem version 0.21.0.