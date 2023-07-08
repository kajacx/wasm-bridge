use js_sys::Function;

#[derive(Debug, Clone)]
pub struct Func {
    pub(crate) function: Function,
}

impl Func {
    pub(crate) fn new(function: Function) -> Self {
        Self { function }
    }
}
