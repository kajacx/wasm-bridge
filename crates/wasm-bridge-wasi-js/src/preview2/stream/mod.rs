use wasm_bridge::component::Linker;
use wasm_bridge::Result;

pub(crate) const STDIN_IDENT: u32 = 0;
pub(crate) const STDOUT_IDENT: u32 = 1;
pub(crate) const STDERR_IDENT: u32 = 2;

mod input_stream;
pub(crate) use input_stream::void_stream;
pub use input_stream::InputStream;

mod output_stream;
pub use output_stream::OutputStream;
pub(crate) use output_stream::{console_error_stream, console_log_stream, voiding_stream};

use super::WasiView;

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    input_stream::add_to_linker(linker)?;
    output_stream::add_to_linker(linker)?;
    Ok(())
}
