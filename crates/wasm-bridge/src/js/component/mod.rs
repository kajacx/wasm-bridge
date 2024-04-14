#[allow(clippy::module_inception)]
mod component;
pub use component::*;

mod linker;
pub use linker::*;

mod instance;
pub use instance::*;

mod exports;
pub use exports::*;

mod func;
pub use func::*;

mod typed_func;
pub use typed_func::*;

mod make_closure;
pub use make_closure::*;

mod resources;
pub use resources::*;

mod component_loader;
pub(crate) use component_loader::*;

pub use wasm_bridge_macros::bindgen_js as bindgen;
pub use wasm_bridge_macros::flags_js as flags;

pub use wasm_bridge_macros::LiftJs;
pub use wasm_bridge_macros::LowerJs;
pub use wasm_bridge_macros::SizeDescription;

pub mod __internal {
    pub use anyhow;

    // #[cfg(feature = "async")]
    // pub use wasm_bridge_macros::async_trait;
    // pub use async_trait::async_trait;

    // From https://github.com/bytecodealliance/wasmtime/blob/v15.0.1/crates/wasmtime/src/component/func/typed.rs#L1791-L1806
    /// Format the specified bitflags using the specified names for debugging
    pub fn format_flags(
        bits: &[u32],
        names: &[&str],
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        f.write_str("(")?;
        let mut wrote = false;
        for (index, name) in names.iter().enumerate() {
            if ((bits[index / 32] >> (index % 32)) & 1) != 0 {
                if wrote {
                    f.write_str("|")?;
                } else {
                    wrote = true;
                }

                f.write_str(name)?;
            }
        }
        f.write_str(")")
    }
}
