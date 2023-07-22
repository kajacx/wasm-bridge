use super::*;
use crate::Result;

#[derive(Debug, Default)]
pub struct WasiCtxBuilder {
    stdout: Option<FunctionWithDrop>,
    stderr: Option<FunctionWithDrop>,
}

impl WasiCtxBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self, _table: &mut Table) -> Result<WasiCtx> {
        Ok(WasiCtx::new(self.stdout, self.stderr))
    }

    pub fn inherit_stdin(self) -> Self {
        self // TODO: this could (in theory) be implemented with prompt, but no one probably wants that
    }

    pub fn inherit_stdout(self) -> Self {
        let console_log = js_sys::eval(
            "(bytes) => {console.log((new TextDecoder()).decode(bytes)); return bytes.byteLength;}",
        )
        .expect("eval console.log bytes");

        Self {
            stdout: Some(FunctionWithDrop::from_js_function(console_log.into())),
            ..self
        }
    }

    pub fn inherit_stderr(self) -> Self {
        let console_error = js_sys::eval(
            "(bytes) => {console.error((new TextDecoder()).decode(bytes)); return bytes.byteLength;}",
        )
        .expect("eval console.error bytes");

        Self {
            stderr: Some(FunctionWithDrop::from_js_function(console_error.into())),
            ..self
        }
    }

    pub fn inherit_stdio(self) -> Self {
        self.inherit_stdin().inherit_stdout().inherit_stderr()
    }
}
