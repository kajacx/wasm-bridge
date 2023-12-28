use crate::preview2::{clocks, WasiView};
use wasm_bridge::component::Linker;
use wasm_bridge::Result;

use super::{cli, random, stream};

pub fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    stream::add_to_linker(linker)?;
    random::add_to_linker(linker)?;
    clocks::add_to_linker(linker)?;
    cli::add_to_linker(linker)?;

    Ok(())
}

wasm_bridge::component::bindgen!({
    // path: "wit/deps/cli/reactor.wit",
    world: "wasi:cli/command",
    tracing: true,
    async: true,
    with: {
        "wasi:cli/environment": crate::preview2::bindings::cli::environment,
    }
});
