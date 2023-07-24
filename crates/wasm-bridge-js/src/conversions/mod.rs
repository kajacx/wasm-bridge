mod from_js_value;
mod into_closure;
mod to_js_value;

#[cfg(test)]
mod from_js_value_tests;

pub use from_js_value::*;
pub use into_closure::*;
pub use to_js_value::*;

/// Possible runtime values that a WebAssembly module can either consume or
/// produce.
#[derive(Debug, Clone)]
pub enum Val {
    // NB: the ordering here is intended to match the ordering in
    // `ValType` to improve codegen when learning the type of a value.
    /// A 32-bit integer
    I32(i32),

    /// A 64-bit integer
    I64(i64),

    /// A 32-bit float.
    ///
    /// Note that the raw bits of the float are stored here, and you can use
    /// `f32::from_bits` to create an `f32` value.
    F32(u32),

    /// A 64-bit float.
    ///
    /// Note that the raw bits of the float are stored here, and you can use
    /// `f64::from_bits` to create an `f64` value.
    F64(u64),
    // /// A 128-bit number
    // V128(u128),
    // /// A first-class reference to a WebAssembly function.
    // ///
    // /// `FuncRef(None)` is the null function reference, created by `ref.null
    // /// func` in Wasm.
    // FuncRef(Option<Func>),

    // /// An `externref` value which can hold opaque data to the Wasm instance
    // /// itself.
    // ///
    // /// `ExternRef(None)` is the null external reference, created by `ref.null
    // /// extern` in Wasm.
    // ExternRef(Option<ExternRef>),
}

impl Val {
    pub fn i32(&self) -> Option<i32> {
        match self {
            Val::I32(i) => Some(*i),
            Val::I64(i) => Some(*i as i32),
            Val::F32(i) => Some(f32::from_bits(*i) as i32),
            Val::F64(i) => Some(f64::from_bits(*i) as i32),
            _ => None,
        }
    }

    pub fn f32(&self) -> Option<f32> {
        match self {
            Val::I32(i) => Some(*i as f32),
            Val::I64(i) => Some(*i as f32),
            Val::F32(i) => Some(f32::from_bits(*i)),
            Val::F64(i) => Some(f64::from_bits(*i as u64) as f32),
            _ => None,
        }
    }
}

impl From<f32> for Val {
    fn from(value: f32) -> Self {
        Val::F32(f32::to_bits(value))
    }
}

impl From<i32> for Val {
    fn from(value: i32) -> Self {
        Val::I32(value)
    }
}
