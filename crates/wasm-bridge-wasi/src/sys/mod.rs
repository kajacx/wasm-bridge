pub use wasmtime_wasi::*;

pub fn add_to_linker_async<T>(
    linker: &mut wasm_bridge::component::Linker<T>,
) -> wasm_bridge::Result<()>
where
    T: super::WasiView,
{
    wasmtime_wasi::add_to_linker_async(&mut linker.0)
}
