use bytes::{Buf, Bytes};
use wasm_bridge::{
    async_trait,
    component::{Component, Linker},
    wasi, Config, Engine, Result, Store,
};

use wasm_bridge::wasi::preview2::*;

use std::{
    io::Write,
    sync::{Arc, Mutex},
};

wasm_bridge::component::bindgen!({
    path: "./io_redirect.wit",
    world: "io-redirect",
    async: true,
});

struct State {
    table: Table,
    wasi: WasiCtx,
}

impl WasiView for State {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
pub async fn test() -> Result<()> {
    use tracing_subscriber::prelude::*;
    #[cfg(target_arch = "wasm32")]
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true) // Only partially supported across browsers
        .without_time()
        .with_writer(tracing_web::MakeConsoleWriter); // write events to the console

    #[cfg(not(target_arch = "wasm32"))]
    let fmt_layer = tracing_subscriber::fmt::layer().with_ansi(true);

    tracing_subscriber::registry().with(fmt_layer).init();
    const GUEST_BYTES: &[u8] = include_bytes!("../../../target/wasm32-wasi/debug/io_guest.wasm");

    no_config(GUEST_BYTES).await?;
    inherit(GUEST_BYTES).await?;
    capture(GUEST_BYTES).await?;

    Ok(())
}

async fn no_config(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let mut table = Table::new();
    let wasi = WasiCtxBuilder::new().build(&mut table)?;

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, State { table, wasi });

    let component = Component::new(store.engine(), component_bytes)?;

    let mut linker = Linker::new(store.engine());
    wasi::preview2::command::add_to_linker(&mut linker)?;

    let (instance, _) = IoRedirect::instantiate_async(&mut store, &component, &linker).await?;

    let result = instance.call_readln_from_stdin(&mut store).await?;
    assert_eq!(result, None);

    instance
        .call_writeln_to_stdout(&mut store, "NO_PRINT")
        .await?;

    instance
        .call_writeln_to_stderr(&mut store, "NO_PRINT")
        .await?;

    Ok(())
}

async fn inherit(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let mut table = Table::new();
    let wasi = WasiCtxBuilder::new().inherit_stdio().build(&mut table)?;

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, State { table, wasi });

    let component = Component::new(&store.engine(), &component_bytes)?;

    let mut linker = Linker::new(store.engine());
    wasi::preview2::command::add_to_linker(&mut linker)?;

    let (instance, _) = IoRedirect::instantiate_async(&mut store, &component, &linker).await?;

    // Cannot really read a line in js when inheriting
    // let result = instance.call_readln_from_stdin(&mut store).await?;
    // assert_eq!(result, None);

    instance
        .call_writeln_to_stdout(&mut store, "PRINT_OUT_1")
        .await?;

    instance
        .call_writeln_to_stderr(&mut store, "PRINT_ERR_1")
        .await?;

    Ok(())
}

async fn capture(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true);

    let out_bytes = Arc::new(Mutex::new(Vec::<u8>::new()));
    let out_stream = OutStream {
        data: out_bytes.clone(),
        max: 3,
    };

    let err_bytes = Arc::new(Mutex::new(Vec::<u8>::new()));
    let err_stream = OutStream {
        data: err_bytes.clone(),
        max: 3,
    };

    let in_bytes = "PRINT_IN_2".to_string().into_bytes();
    let in_stream = InStream {
        data: in_bytes,
        offset: 0,
        max: 3,
    };

    let mut table = Table::new();
    let wasi = WasiCtxBuilder::new()
        .stdin(in_stream, IsATTY::No)
        .stdout(out_stream, IsATTY::No)
        .stderr(err_stream, IsATTY::No)
        .build(&mut table)?;

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, State { table, wasi });

    let component = Component::new(&store.engine(), &component_bytes)?;

    let mut linker = Linker::new(store.engine());
    wasi::preview2::command::add_to_linker(&mut linker)?;

    let (instance, _) = IoRedirect::instantiate_async(&mut store, &component, &linker).await?;

    let result = instance.call_readln_from_stdin(&mut store).await?;
    assert_eq!(result, Some("PRINT_IN_2".into()));

    let result = instance.call_readln_from_stdin(&mut store).await?;
    assert_eq!(result, None);

    instance
        .call_writeln_to_stdout(&mut store, "PRINT_OUT_2")
        .await?;
    instance
        .call_writeln_to_stdout(&mut store, "NO_PRINT")
        .await?; // Test that output is not duplicated to stdout

    instance
        .call_writeln_to_stderr(&mut store, "PRINT_ERR_2")
        .await?;
    instance
        .call_writeln_to_stdout(&mut store, "NO_PRINT")
        .await?;

    let text = String::from_utf8(out_bytes.try_lock().unwrap().clone())?;
    assert!(text.contains("PRINT_OUT_2"), "stdout is captured");

    let text = String::from_utf8(err_bytes.try_lock().unwrap().clone())?;
    assert!(text.contains("PRINT_ERR_2"), "stderr is captured");

    Ok(())
}

struct OutStream {
    data: Arc<Mutex<Vec<u8>>>,
    max: usize,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bridge::async_trait]
impl OutputStream for OutStream {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    async fn writable(&self) -> Result<()> {
        Ok(())
    }

    async fn write(&mut self, buf: &[u8]) -> Result<(usize, StreamStatus)> {
        let amount = buf.len().min(self.max);
        self.data.try_lock().unwrap().extend(&buf[..amount]);
        Ok((amount as usize, StreamStatus::Open))
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[wasm_bridge::async_trait]
impl HostOutputStream for OutStream {
    fn write(&mut self, bytes: Bytes) -> Result<(usize, StreamState), wasm_bridge::Error> {
        let amount = bytes.len().min(self.max);
        self.data.try_lock().unwrap().extend(&bytes[..amount]);
        Ok((amount, StreamState::Open))
    }

    async fn ready(&mut self) -> Result<(), wasm_bridge::Error> {
        Ok(())
    }
}

struct InStream {
    data: Vec<u8>,
    offset: usize,
    max: usize,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bridge::async_trait]
impl InputStream for InStream {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    async fn readable(&self) -> Result<()> {
        Ok(())
    }

    async fn read(&mut self, buf: &mut [u8]) -> Result<(u64, StreamStatus)> {
        let len = buf.len().min(self.data.len() - self.offset).min(self.max);
        let from_slice = &self.data[self.offset..(self.offset + len)];

        (&mut buf[..len]).copy_from_slice(from_slice);
        self.offset += len;

        Ok((
            len as _,
            if self.data.len() == self.offset {
                StreamStatus::Ended
            } else {
                StreamStatus::Open
            },
        ))
    }

    async fn num_ready_bytes(&self) -> Result<u64> {
        Ok((self.data.len() - self.offset) as _)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[wasm_bridge::async_trait]
impl HostInputStream for InStream {
    fn read(&mut self, size: usize) -> Result<(Bytes, StreamState), wasm_bridge::Error> {
        let len = size.min(self.data.len() - self.offset).min(self.max);

        let from_slice = self.data[self.offset..(self.offset + len)].to_vec();

        let buf = Bytes::from(from_slice);
        self.offset += len;

        Ok((
            buf,
            if self.data.len() == self.offset {
                StreamState::Closed
            } else {
                StreamState::Open
            },
        ))
    }

    async fn ready(&mut self) -> Result<(), wasm_bridge::Error> {
        Ok(())
    }
}
