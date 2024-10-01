use wasmtime::*;

fn main() -> wasmtime::Result<()> {
    let engine = Engine::default();
    // Can also use .wasm file but the wasm file havwe multiple imports and for simplicity, we are using wat here
    let wat_add = r#"
        (module
            (type $t0 (func (param i32 i32) (result i32)))
            (func $addTwo (export "addTwo") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
            (i32.add
                (local.get $p0)
                (local.get $p1))))
    "#;

    // Can also use .wasm file but the wasm file havwe multiple imports and for simplicity, we are using wat here
    let wat_sub = r#"
        (module
            (type $t0 (func (param i32 i32) (result i32)))
            (func $subTwo (export "subTwo") (type $t0) (param $p0 i32) (param $p1 i32) (result i32)
            (i32.sub
                (local.get $p0)
                (local.get $p1))))
    "#;

    let module_add = Module::new(&engine, wat_add)?;
    
    let module_sub = Module::new(&engine, wat_sub)?;

    // Loading via .wasm file
    // let module_add = Module::new(&engine, './add.wasm')?;
    // let module_sub = Module::new(&engine, './sub.wasm')?;
    
    // Host functionality can be arbitrary Rust functions and is provided
    // to guests through a `Linker`.
    // let mut linker = Linker::new(&engine);
    // linker.func_wrap("add", "host_func", |caller: Caller<'_, u32>, param: i32| {
    //     println!("Got {} from WebAssembly", param);
    //     println!("my host state is: {}", caller.data());
    // })?;

    let mut store: Store<u32> = Store::new(&engine, 4);

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