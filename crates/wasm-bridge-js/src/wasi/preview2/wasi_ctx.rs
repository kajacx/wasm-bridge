use js_sys::Function;

use crate::DropHandler;

#[derive(Debug, Default)]
pub struct WasiCtx {
    stdout: Option<FunctionWithDrop>,
    stderr: Option<FunctionWithDrop>,
}

impl WasiCtx {
    pub(crate) fn new(stdout: Option<FunctionWithDrop>, stderr: Option<FunctionWithDrop>) -> Self {
        Self { stdout, stderr }
    }

    pub(crate) fn stdout(&self) -> Option<&Function> {
        self.stdout.as_ref().map(|f| &f.0)
    }

    pub(crate) fn stderr(&self) -> Option<&Function> {
        self.stderr.as_ref().map(|f| &f.0)
    }
}

#[derive(Debug)]
pub(crate) struct FunctionWithDrop(Function, Option<DropHandler>);

impl FunctionWithDrop {
    pub fn from_js_function(function: Function) -> Self {
        assert!(function.is_function(), "FunctionWithDrop is function");
        Self(function, None)
    }
}
