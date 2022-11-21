use wasmer::{imports, EngineBuilder, Instance, Module, Store};

fn main() {
    let wasm_bytes = include_bytes!("../../target/wasm32-unknown-unknown/debug/wasm.wasm");

    let engine = EngineBuilder::new(wasmer_compiler_singlepass::Singlepass::default()); // crashes
    //let engine = EngineBuilder::new(wasmer::Cranelift::default()); // works
    let mut store = Store::new(engine);
    let module = Module::new(&store, wasm_bytes).unwrap();
    let imports = imports! {};
    let instance = Instance::new(&mut store, &module, &imports).unwrap();
    let func = instance.exports.get_function("deserialize_msgpack").unwrap(); // crashes
    //let func = instance.exports.get_function("deserialize_json").unwrap(); // works
    func.call(&mut store, &[]).unwrap();
}

/*
// The same issue exists in WASMER 2.3:
fn main() {
    let wasm_bytes = include_bytes!("../../target/wasm32-unknown-unknown/debug/wasm.wasm");

    let compiler = wasmer::Singlepass::default(); // crashes
    let compiler = wasmer::Cranelift::default(); // works

    let engine = wasmer::Universal::new(compiler).engine();
    let store = Store::new(&engine);
    let module = Module::new(&store, wasm_bytes).unwrap();
    let imports = imports! {};
    let instance = Instance::new(&module, &imports).unwrap();
    let func = instance.exports.get_native_function::<(), ()>("deserialize").unwrap();
    func.call().unwrap();
}
*/
