# Branch info

There are two branches in this repo - `main` and `multi`. 

## `main` branch
`main` branch serves an example of two separate WASM modules like add and sub which can be debugged separately. 

## `multi` branch 
`multi` branch serves an example of an host application which loads three WASM modules like add, sub, fib and debugs the host application which eventually steps into the source code of the three modules when breakpoints are set. 

### How to build .wasm files
`cd ./host/src/wasm_add`
`cargo build -p wasm-add --target wasm32-unknown-unknown`

`cd ./host/src/wasm_sub`
`cargo build -p wasm-sub --target wasm32-unknown-unknown`

`cd ./host/src/wasm_fib`
`cargo build -p wasm-fib --target wasm32-unknown-unknown`

### How to build host app
`cd ./host`
`cargo build`

### Run host app 
`cargo run`

### launch debugger in host app
`Run and debug` in the Vs code extension and use the launch.json in this repo to debug 