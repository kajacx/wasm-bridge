mod monotonic_clock;
mod wall_clock;

pub use monotonic_clock::*;
pub use wall_clock::*;

use crate::component::Linker;
use crate::Result;

use super::WasiView;

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    monotonic_clock::add_to_linker(linker)?;
    wall_clock::add_to_linker(linker)?;
    Ok(())
}
