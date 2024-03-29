use wasm_bridge::component::Linker;
use wasm_bridge::Result;

use super::WasiView;

#[derive(
    Debug,
    wasm_bridge_macros::SizeDescription,
    // wasm_bridge_macros::LiftJs,
    wasm_bridge_macros::LowerJs,
)]
#[component(variant)]
pub enum StreamError {
    LastOperationFailed(anyhow::Error),
    Closed,
    Trap(anyhow::Error),
}

pub type StreamResult<T> = Result<T, StreamError>;

mod input_stream;
pub(crate) use input_stream::void_stream;
pub use input_stream::{HostInputStream, StdinStream};

mod output_stream;
pub(crate) use output_stream::{console_error_stream, console_log_stream, voiding_stream};
pub use output_stream::{HostOutputStream, StdoutStream};

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    input_stream::add_to_linker(linker)?;
    output_stream::add_to_linker(linker)?;
    Ok(())
}

pub trait Subscribe {
    fn ready(&mut self);
}
