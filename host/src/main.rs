use anyhow::Context;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::bindings::Imports as WasiImports;
use wasmtime_wasi::preview2::{WasiCtx, WasiCtxBuilder, WasiView};

wasmtime::component::bindgen!({
    path: "./hello.wit",
    world: "hello",
    //async: true
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

#[derive(Copy, Clone)]
struct Env {}

//#[async_trait::async_trait]
impl component::guest::env::Host for Env {
    fn abort(&mut self, message: String, filename: String) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn assert(&mut self, condition: i32, message: String) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

#[derive(Copy, Clone)]
struct Mamoru {}

impl component::guest::mamoru::Host for Mamoru {
    fn query(&mut self, query: String) -> Result<String, anyhow::Error> {
        Ok("query".to_string())
    }

    fn report(&mut self, incident_json: String) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn http(&mut self, request_json: String) -> Result<String, anyhow::Error> {
        Ok("http".to_string())
    }

    fn parameter(&mut self, key: String) -> Result<String, anyhow::Error> {
        Ok("parameter".to_string())
    }
    fn u256_from_str(&mut self, str_: String) -> Result<u64, anyhow::Error> {
        Ok(1 as u64)
    }
}

#[derive(Copy, Clone)]
struct MamoruStorage {}

impl component::guest::mamoru_storage::Host for MamoruStorage {
    fn open_external(&mut self, input: String) -> Result<i32, anyhow::Error> {
        Ok(1 as i32)
    }
    fn get(&mut self, input: String) -> Result<Vec<String>, anyhow::Error> {
        Ok(vec!["hello".to_string()])
    }
    fn set(&mut self, input: String) -> Result<Vec<String>, anyhow::Error> {
        Ok(vec!["hello".to_string()])
    }
    fn delete(&mut self, input: String) -> Result<(), anyhow::Error> {
        Ok(())
    }
    fn keys(&mut self, input: String) -> Result<Vec<String>, anyhow::Error> {
        Ok(vec!["hello".to_string()])
    }
}

#[derive(Copy, Clone)]
struct MamoruEvm {}

impl component::guest::mamoru_evm::Host for MamoruEvm {
    fn parse_tx_input(&mut self, abi: String, input: String) -> Result<u64, anyhow::Error> {
        Ok(1 as u64)
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
    let (app, _instance) =
        Hello::instantiate(&mut store, &component, &linker).expect("It can not be instantiated");
    /*
    let msg = app
        .call_hello_world(&mut store)
        .await
        .context("Failed to call add function")?;
    println!("msg={:?}", msg);
    */
    /*
    app.call_entrypoint(&mut store)
        .await
        .context("Checking message ");
    */
    let _ = app
        .call_entrypoint(&mut store)
        .context("Checking message...");
    Ok(())
}
