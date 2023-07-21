use super::*;

pub trait WasiView {
    fn table(&self) -> &Table;

    fn table_mut(&mut self) -> &mut Table;

    fn ctx(&self) -> &WasiCtx;

    fn ctx_mut(&mut self) -> &mut WasiCtx;
}
