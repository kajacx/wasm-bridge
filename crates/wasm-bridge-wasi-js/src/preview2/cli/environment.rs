use anyhow::Ok;
use wasm_bridge::component::Linker;
use wasm_bridge::Result;

use crate::preview2::wasi;
use crate::preview2::WasiView;

// wasm_bridge::component::bindgen!({
//     path: "src/preview2/wits/environment.wit",
//     world: "exports"
// });

impl<T: WasiView + 'static> wasi::cli::environment::Host for T {
    fn get_environment(&mut self) -> Result<Vec<(String, String)>> {
        Ok(vec![])
    }

    fn get_arguments(&mut self) -> Result<Vec<String>> {
        Ok(vec![])
    }

    fn initial_cwd(&mut self) -> Result<Option<String>> {
        Ok(None)
    }
}

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    wasi::cli::environment::add_to_linker(linker, |d| d)
}
