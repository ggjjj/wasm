# Branch info

There are two branches in this repo - `main` and `multi`. 
`main` branch serves an example of two separate WASM modules like add and sub which can be debugged separately. 
`multi` branch serves an example of an host application which loads two WASM modules like add and sub from WAT and tests a simple application graph with two WASM modules connected sequentially. It also contains two .wasm files (add.wasm and sub.wasm) from add.rs and sub.rs which are not used currently
