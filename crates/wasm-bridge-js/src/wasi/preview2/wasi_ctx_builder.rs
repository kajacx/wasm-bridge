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
}
