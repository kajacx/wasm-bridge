use anyhow::bail;
use js_sys::Object;

use crate::preview2::{clocks, WasiView};
use wasm_bridge::component::Linker;
use wasm_bridge::{Result, StoreContextMut};

static WASI_IMPORTS_STR: &str =
    include_str!("../../../../resources/transformed/preview2-shim/bundled.js");

const STDIN_IDENT: u32 = 0;
const STDOUT_IDENT: u32 = 1;
const STDERR_IDENT: u32 = 2;

pub fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    // Default imports
    // TODO: this doesn't work?
    // linker.set_wasi_imports(get_imports());

    // Overrides
    linker.instance("wasi:io/streams")?.func_wrap(
        "read",
        |data: StoreContextMut<T>, (id, max_bytes): (u32, u64)| {
            if id != STDIN_IDENT {
                bail!("unexpected read stream id: {id}");
            }

            let mut bytes = vec![0u8; max_bytes as usize];

            let (bytes_written, stream_ended) = data.ctx_mut().stdin().read(&mut bytes)?;

            bytes.truncate(bytes_written as _);

            Ok((bytes, stream_ended))
        },
    )?;

    linker.instance("wasi:io/streams")?.func_wrap(
        "write",
        |data: StoreContextMut<T>, (id, buffer): (u32, Vec<u8>)| {
            let bytes_written = match id {
                STDOUT_IDENT => data.ctx_mut().stdout().write(&buffer)?,

                STDERR_IDENT => data.ctx_mut().stderr().write(&buffer)?,

                id => bail!("unexpected write stream id: {id}"),
            };
            Ok(bytes_written)
        },
    )?;

    linker.instance("wasi:random/random")?.func_wrap(
        "get-random-bytes",
        |data: StoreContextMut<T>, (len,): (u64,)| {
            let random = data.ctx_mut().random();
            let mut bytes = vec![0u8; len as _];
            random.fill_bytes(&mut bytes);
            Ok(bytes)
        },
    )?;

    linker
        .instance("wasi:random/random")?
        .func_wrap("get-random-u64", |data: StoreContextMut<T>, (): ()| {
            Ok(data.ctx_mut().random().next_u64())
        })?;

    clocks::add_to_linker(linker)?;

    Ok(())
}

fn get_imports() -> Object {
    let imports = js_sys::eval(WASI_IMPORTS_STR).expect("eval bundled wasi imports");

    assert!(imports.is_object(), "wasi imports must be an object");

    imports.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn should_get_imports() {
        let _ = get_imports();
    }
}
