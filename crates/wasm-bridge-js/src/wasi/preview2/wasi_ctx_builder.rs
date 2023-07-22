use super::*;
use crate::Result;

#[derive(Debug, Clone, Default)]
pub struct WasiCtxBuilder {}

impl WasiCtxBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(self, _table: &mut Table) -> Result<WasiCtx> {
        Ok(WasiCtx::new())
    }

    pub fn inherit_stdin(self) -> Self {
        self // TODO: this could (in theory) be implemented with prompt, but no one probably wants that
    }

    pub fn inherit_stdout(self) -> Self {
        self
    }

    pub fn inherit_stderr(self) -> Self {
        self
    }

    pub fn inherit_stdio(self) -> Self {
        self.inherit_stdin().inherit_stdout().inherit_stderr()
    }
}
