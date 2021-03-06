use std::process::Command;
use std::error::Error;
use std::fs;
use wasmer::{Instance, Module, Store};
use wasmer_wasi::WasiState;

fn main() -> Result<(), Box<dyn Error>> {
    // compile c source code in hello-world.c to hello-world.wasm
    Command::new("make").output().expect("Failed to run make");

    let wasm_bytes = fs::read("./hello-world.wasm")?; 
    let store = Store::default();
    let module = Module::new(&store, wasm_bytes)?;
    let import_object = WasiState::new("some_name").finalize()?.import_object(&module)?;
    let instance = Instance::new(&module, &import_object)?;

    let start = instance.exports.get_function("_start")?;
    start.call(&[])?;

    Ok(())
}