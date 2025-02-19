# Branch info

There are two branches in this repo - `main` and `multi`. 
`main` branch serves an example of two separate WASM modules like add and sub which can be debugged separately. 
`multi` branch serves an example of an host application which loads two WASM modules like add and sub from WAT and tests a simple application graph with two WASM modules connected sequentially. It also contains two .wasm files (add.wasm and sub.wasm) from add.rs and sub.rs which are not used currently

# Installation
1. Install Rust and wasmtime
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
```
curl https://wasmtime.dev/install.sh -sSf | bash
```

2. Set up target
```
rustup target add wasm32-wasip1
```

3. Compile add.rs and run
```
rustc --target=wasm32-wasip1 -O add.rs -o add.wasm
rustc --target=wasm32-wasip1 -O sub.rs -o sub.wasm
wasmtime add.wasm
wasmtime sub.wasm
```

4. Install `wasm-pack` for JS bindings
```
cargo install wasm-pack
```

5. Test the javascript code
```
fetch('wasm_add/target/wasm32-unknown-unknown/release/wasm_add.wasm')
  .then(response => response.arrayBuffer())
  .then(bytes => WebAssembly.instantiate(bytes))
  .then(({ instance }) => {
    console.log(instance.exports.add(5, 10)); // Output: 15
  });

```