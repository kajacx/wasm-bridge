use super::*;
use crate::Result;

pub struct WasiCtxBuilder {}

impl WasiCtxBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(self, _table: &mut Table) -> Result<WasiCtx> {
        Ok(WasiCtx::new())
    }
}
