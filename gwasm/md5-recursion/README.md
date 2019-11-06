# MD5 Recursion Task

cd to "in" directory and compile:
```bash
cargo rustc --target=wasm32-unknown-emscripten --release -- -C link-args="-s BINARYEN_ASYNC_COMPILATION=0" --verbose && cp ../target/wasm32-unkn
own-emscripten/release/md5* ./
```