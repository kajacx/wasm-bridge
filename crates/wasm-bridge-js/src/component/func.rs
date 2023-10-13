use std::{mem::MaybeUninit, rc::Rc};

use js_sys::{Array, DataView, Function};
use wasm_bindgen::JsValue;

use crate::DropHandle;

#[derive(Debug, Clone)]
pub struct Func {
    pub(crate) function: Function,
    _closures: Rc<[DropHandle]>,
}

impl Func {
    pub(crate) fn new(function: Function, closures: Rc<[DropHandle]>) -> Self {
        Self {
            function,
            _closures: closures,
        }
    }
}
