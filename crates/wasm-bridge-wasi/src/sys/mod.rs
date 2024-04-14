pub use wasmtime_wasi::*;

pub mod command {
    pub fn add_to_linker<T>(
        linker: &mut wasm_bridge::component::Linker<T>,
    ) -> wasm_bridge::Result<()>
    where
        T: super::WasiView,
    {
        wasmtime_wasi::command::add_to_linker(&mut linker.0)
    }
}
