use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};
use wasm_bridge_wasi::*;

use bytes::Bytes;
use std::sync::{Arc, Mutex};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "io-redirect",
    async: true,
});

struct State {
    table: ResourceTable,
    wasi: WasiCtx,
}

impl WasiView for State {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

pub async fn run_test(component_bytes: &[u8]) -> Result<()> {
    no_config(component_bytes).await.unwrap();
    inherit(component_bytes).await.unwrap();
    capture(component_bytes).await.unwrap();

    Ok(())
}

async fn no_config(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let table = ResourceTable::new();
    let wasi = WasiCtxBuilder::new().build();

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, State { table, wasi });

    #[allow(deprecated)]
    let component = Component::new(&store.engine(), &component_bytes).unwrap(); 

    let mut linker = Linker::new(store.engine());
    add_to_linker_async(&mut linker).unwrap();

    let (instance, _) = IoRedirect::instantiate_async(&mut store, &component, &linker).await.unwrap();

    let result = instance.call_readln_from_stdin(&mut store).await.unwrap();
    assert_eq!(result, None);

    instance.call_writeln_to_stdout(&mut store, "NO_PRINT").await.unwrap();
    instance.call_writeln_to_stderr(&mut store, "NO_PRINT").await.unwrap();

    Ok(())
}

async fn inherit(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let table = ResourceTable::new();
    let wasi = WasiCtxBuilder::new().inherit_stdio().build();

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, State { table, wasi });

    #[allow(deprecated)]
    let component = Component::new(&store.engine(), &component_bytes).unwrap(); 

    let mut linker = Linker::new(store.engine());
    add_to_linker_async(&mut linker).unwrap();

    let (instance, _) = IoRedirect::instantiate_async(&mut store, &component, &linker).await.unwrap();

    instance.call_writeln_to_stdout(&mut store, "PRINT_OUT_1").await.unwrap();
    instance.call_writeln_to_stderr(&mut store, "PRINT_ERR_1").await.unwrap();

    Ok(())
}

async fn capture(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let out_bytes = Arc::new(Mutex::new(Vec::<u8>::new()));
    let out_stream = OutStream{ data: out_bytes.clone(), max: 3 };

    let err_bytes = Arc::new(Mutex::new(Vec::<u8>::new()));
    let err_stream = OutStream{ data: err_bytes.clone(), max: 3 };

    let in_bytes = "PRINT_IN_2".to_string().into_bytes();
    let in_stream = InStream { data: in_bytes, offset: 0, max: 3 };

    let table = ResourceTable::new();
    let wasi = WasiCtxBuilder::new()
        .stdin(in_stream)
        .stdout(out_stream)
        .stderr(err_stream)
        .build();

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, State { table, wasi });

    #[allow(deprecated)]
    let component = Component::new(&store.engine(), &component_bytes).unwrap(); 

    let mut linker = Linker::new(store.engine());
    add_to_linker_async(&mut linker).unwrap();

    let (instance, _) = IoRedirect::instantiate_async(&mut store, &component, &linker).await.unwrap();

    let result = instance.call_readln_from_stdin(&mut store).await.unwrap();
    assert_eq!(result, Some("PRINT_IN_2".into()));

    let result = instance.call_readln_from_stdin(&mut store).await.unwrap();
    assert_eq!(result, None);

    instance.call_writeln_to_stdout(&mut store, "PRINT_OUT_2").await.unwrap();
    instance.call_writeln_to_stdout(&mut store, "NO_PRINT").await.unwrap(); // Test that output is not duplicated to stdout

    instance.call_writeln_to_stderr(&mut store, "PRINT_ERR_2").await.unwrap();
    instance.call_writeln_to_stderr(&mut store, "NO_PRINT").await.unwrap();

    let text = String::from_utf8(out_bytes.try_lock().unwrap().clone()).unwrap();
    assert!(text.contains("PRINT_OUT_2"), "stdout is captured");

    let text = String::from_utf8(err_bytes.try_lock().unwrap().clone()).unwrap();
    assert!(text.contains("PRINT_ERR_2"), "stderr is captured");

    assert_eq!(*GLOBAL_STRING.lock().unwrap(), "async fn".to_owned());

    Ok(())
}

#[derive(Clone, Debug)]
struct OutStream { 
    data: Arc<Mutex<Vec<u8>>>,
    max: usize
}

#[wasm_bridge::async_trait]
impl Subscribe for OutStream {
    async fn ready(&mut self) {}
}

impl HostOutputStream for OutStream {
    fn write(&mut self, buf: Bytes) -> StreamResult<()> {
        assert!(buf.len() <= self.max, "We specified to write at most {} bytes at a time.", self.max);
        self.data.try_lock().unwrap().extend(buf);
        StreamResult::Ok(())
    }

    fn flush(&mut self) -> StreamResult<()> {
        StreamResult::Ok(())
    }

    fn check_write(&mut self) -> StreamResult<usize> {
        StreamResult::Ok(self.max)
    }
}

impl StdoutStream for OutStream {
    fn stream(&self) -> Box<(dyn HostOutputStream + 'static)> {
        Box::new((*self).clone())
    }

    fn isatty(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
struct InStream {
    data: Vec<u8>,
    offset: usize,
    max: usize,
}

static GLOBAL_STRING: Mutex<String> = Mutex::new(String::new());

#[wasm_bridge::async_trait]
impl Subscribe for InStream {
    async fn ready(&mut self) {
        *GLOBAL_STRING.lock().unwrap() = "async fn".to_owned();
    }
}

impl HostInputStream for InStream {
     fn read(&mut self, size: usize) -> StreamResult<Bytes> {
        let start = self.offset;
        let len =  (self.data.len() - start).min(self.max).min(size);
        let end = start + len;

        self.offset = end;

        if size > 0 && len == 0 {
            StreamResult::Err(StreamError::Closed)
        } else {
            StreamResult::Ok(Bytes::copy_from_slice(&self.data[start..end]))
        }
    }
}

impl StdinStream for InStream {
    fn stream(&self) -> Box<(dyn HostInputStream + 'static)> {
        Box::new((*self).clone())
    }

    fn isatty(&self) -> bool {
        false
    }
}
