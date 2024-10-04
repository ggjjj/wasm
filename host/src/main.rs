use wasmtime::*;

fn main() -> Result<()> {
    // Load our previously compiled wasm file (built previously with Cargo) and
    // also ensure that we generate debuginfo so this executable can be
    // debugged in lldb.
    let engine = Engine::new(
        Config::new()
            .debug_info(true) // Very essential for breakpoint debugging
    )?;
    let mut store = Store::new(&engine, ());
    
    // Add wasm module
    let module_add = Module::from_file(&engine, "/workspaces/wasm/host/src/wasm_add/target/wasm32-unknown-unknown/debug/add.wasm")?;
    let instance_add = Instance::new(&mut store, &module_add, &[])?;

    // Sub wasm module
    let module_sub = Module::from_file(&engine, "/workspaces/wasm/host/src/wasm_sub/target/wasm32-unknown-unknown/debug/sub.wasm")?;
    let instance_sub = Instance::new(&mut store, &module_sub, &[])?;

    // Add fib module
    let module_fib = Module::from_file(&engine, "/workspaces/wasm/host/src/wasm_fib/target/wasm32-unknown-unknown/debug/fib.wasm")?;
    let instance_fib = Instance::new(&mut store, &module_fib, &[])?;

    // Invoke `add` export
    let add = instance_add.get_typed_func::<(u32, u32), u32>(&mut store, "add")?;
    let result_add = add.call(&mut store, (10, 20))?;
    println!("Add Wasm module: Result of {} + {} = {}", 10, 20, result_add);

    // Invoke `sub` export
    let sub = instance_sub.get_typed_func::<(u32, u32), u32>(&mut store, "sub")?;
    let result_sub = sub.call(&mut store, (result_add, 25))?;
    println!("Sub Wasm module: Result of {} - {} = {}", result_add, 25, result_sub);

    // Invoke `fib` export
    let fib = instance_fib.get_typed_func::<u32, u32>(&mut store, "fib")?;
    println!("Fib Wasm module: Fibonacci of {} = {}", result_sub, fib.call(&mut store, result_sub)?);
    Ok(())
}


/*
use std::fs::File;

use std::io::{self, Read};
use wasmtime::*;

// Function to read a wasm file and return its bytes
fn read_wasm_file(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

// This is a simple example of using wasmtime to instantiate a two WebAssembly modules and call their functions

fn main() -> wasmtime::Result<()> {
    // Set up wasmtime engine with debug info enabled
    let mut config = Config::new();
    config.debug_info(true);
    let engine = Engine::new(&config)?;

    // Can also use .wasm file but the wasm file have multiple imports and for simplicity, we are using wat here
    // let wat_add = r#"
    //     (module
    //         (type $t0 (func (param i32 i32) (result i32)))
    //         (func $addTwo (export "addTwo") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    //         (i32.add
    //             (local.get $p0)
    //             (local.get $p1))))
    // "#;

    // Can also use .wasm file but the wasm file have multiple imports and for simplicity, we are using wat here
    // let wat_sub = r#"
    //     (module
    //         (type $t0 (func (param i32 i32) (result i32)))
    //         (func $subTwo (export "subTwo") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
    //         (i32.sub
    //             (local.get $p0)
    //             (local.get $p1))))
    // "#;

    // let module_add = Module::new(&engine, wat_add)?;
    // let module_sub = Module::new(&engine, wat_sub)?;

    // Loading via .wasm file
    let wasm_bytes_add = read_wasm_file("/workspaces/wasm/add.wasm")?;
    let wasm_bytes_sub = read_wasm_file("/workspaces/wasm/sub.wasm")?;
    let module_add = Module::new(&engine, wasm_bytes_add)?;
    let module_sub = Module::new(&engine, wasm_bytes_sub)?;
    
    // We are not trying host and guest functions here but it can be done by using Linker
    
    // Host functionality can be arbitrary Rust functions and is provided
    // to guests through a `Linker`.
    // let mut linker = Linker::new(&engine);
    // linker.func_wrap("add", "host_func", |caller: Caller<'_, u32>, param: i32| {
    //     println!("Got {} from WebAssembly", param);
    //     println!("my host state is: {}", caller.data());
    // })?;

    let mut store = Store::new(&engine, 4);

    // Instantiation of a module requires specifying its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    // let instance_add = linker.instantiate(&mut store, &module_add)?;

    let instance_add = Instance::new(&mut store, &module_add, &[])?;
    let add_two = instance_add.get_typed_func::<(i32, i32), i32>(&mut store, "addTwo")?;

    // let instance_sub = linker.instantiate(&mut store, &module_sub)?;
    let instance_sub = Instance::new(&mut store, &module_sub, &[])?;
    let sub_two = instance_sub.get_typed_func::<(i32, i32), i32>(&mut store, "subTwo")?;

    let result_add = add_two.call(&mut store, (1, 2))?;
    println!("Result from Add module: {}", result_add);
    
    let result_sub = sub_two.call(&mut store, (result_add, 2))?;
    println!("Result from Sub module: {}", result_sub);

    Ok(())
}
*/