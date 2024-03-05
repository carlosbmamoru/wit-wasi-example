use anyhow::Context;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::bindings::Imports as WasiImports;
use wasmtime_wasi::preview2::{WasiCtx, WasiCtxBuilder, WasiView};

wasmtime::component::bindgen!({
    path: "hello.wit",
    world: "hello",
    async: true
});

//TODO specify wasm file, it would search locally
const PLUGIN_FILE: &str = "hello.wasm";

struct ServerWasiView {
    table: ResourceTable,
    ctx: WasiCtx,
}

impl ServerWasiView {
    fn new() -> Self {
        let table = ResourceTable::new();
        let ctx = WasiCtxBuilder::new().inherit_stdio().build();
        Self { table, ctx }
    }
}

impl WasiView for ServerWasiView {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Create an 'engine', which is a struct that executes Wasm code
    let mut engine_config = Config::new();
    engine_config.wasm_component_model(true);
    engine_config.async_support(true);
    let engine = Engine::new(&engine_config).unwrap();

    // Create a 'linker', which associates WIT interfaces with concrete implementations
    let mut linker: Linker<ServerWasiView> = Linker::new(&engine);
    let component = Component::from_file(&engine, PLUGIN_FILE).expect("Could not find plugin");
    // and then
    WasiImports::add_to_linker(&mut linker, |ctx| ctx)?;
    //Plugin::add_to_linker(&mut linker, |context| &mut context.logger).unwrap();

    let server_wasi_ctx = ServerWasiView::new();
    let mut store = Store::new(&engine, server_wasi_ctx);
    let (app, _instance) = Hello::instantiate_async(&mut store, &component, &linker)
        .await
        .expect("It can not be instantiated");
    let msg = app
        .call_hello_world(&mut store)
        .await
        .context("Failed to call add function")?;
    println!("msg={:?}", msg);
    Ok(())
}
