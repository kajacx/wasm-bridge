use super::*;

pub trait WasiView {
    fn table(&mut self) -> &mut Table;
    fn ctx(&mut self) -> &mut WasiCtx;
}
