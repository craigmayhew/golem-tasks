# Golem

This briefly shows you how to install golem, rust, compile golem tasks and deploy to mainnet.

Tasks:
 - [md5 recursion](gwasm/md5-recursion/README.md)

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

1. Every subtask needs a directory within the "in" directory or they timeout. A useful command to do this: `mkdir subtask{0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20}`
2. The total timeout needs to be atleast twice the timeout of a single subtask or Golem can't retry them.

### Emscripten

1. Error "Target feature 'atomics' used in" some_file.o is disallowed" means you need to add the flag "-s USE_PTHREADS=1"

## ToDo

1. It would be nice to have a simple script that generates the task.json file, especially the subtasks.
2. Get Golem going on a meaningful problem such as 33.
3. Is there an upper limit to the number of subtasks we can place on the network? i.e. what if it's a billion, and then what happens?

## Hacks

Generate json for subtasks using javascript
```js
var subtasks = "";
var numbers = '';

for (var i=0; i<100; i++) {
   numbers += i + ',';
   subtasks += 
   '"subtask' + i + '": {' +  '"exec_args": ["seed' + i + '"],"output_file_paths":["out.txt"]},';
}

console.log(subtasks);
console.log(numbers);
```
