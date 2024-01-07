mod monotonic_clock;
mod wall_clock;

pub(crate) use monotonic_clock::default_monotonic_clock;
pub use monotonic_clock::HostMonotonicClock;
pub(crate) use wall_clock::real_wall_clock;
pub use wall_clock::HostWallClock;

use wasm_bridge::component::Linker;
use wasm_bridge::Result;

use super::WasiView;

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    monotonic_clock::add_to_linker(linker)?;
    wall_clock::add_to_linker(linker)?;
    Ok(())
}
