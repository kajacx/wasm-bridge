use super::*;

pub trait WasiView {
    fn table(&mut self) -> &mut ResourceTable;

    fn ctx(&mut self) -> &mut WasiCtx;
}
