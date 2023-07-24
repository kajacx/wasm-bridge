use super::*;
use crate::Result;

#[derive(Default)]
pub struct WasiCtxBuilder {
    stdin: Option<Box<dyn InputStream>>,
    stdout: Option<Box<dyn OutputStream>>,
    stderr: Option<Box<dyn OutputStream>>,
}

impl WasiCtxBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build(self, _table: &mut Table) -> Result<WasiCtx> {
        Ok(WasiCtx::new(self.stdin, self.stdout, self.stderr))
    }

    pub fn set_stdout(self, out: impl OutputStream + 'static) -> Self {
        Self {
            stdout: Some(Box::new(out)),
            ..self
        }
    }

    pub fn set_stderr(self, err: impl OutputStream + 'static) -> Self {
        Self {
            stderr: Some(Box::new(err)),
            ..self
        }
    }

    pub fn inherit_stdin(self) -> Self {
        // TODO: could be implemented at least on node, but readline is asynchronous
        self
    }

    pub fn inherit_stdout(self) -> Self {
        Self {
            stdout: Some(Box::new(console_log_stream())),
            ..self
        }
    }

    pub fn inherit_stderr(self) -> Self {
        Self {
            stderr: Some(Box::new(console_error_stream())),
            ..self
        }
    }

    pub fn inherit_stdio(self) -> Self {
        self.inherit_stdin().inherit_stdout().inherit_stderr()
    }
}
