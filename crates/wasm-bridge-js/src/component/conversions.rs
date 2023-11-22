use js_sys::{Array, DataView, Uint8Array};
use wasm_bindgen::JsValue;

pub struct LowerContext {}

pub struct Encoder<'a> {
    dv: &'a mut DataView,
    offset: usize,
}

impl<'a> Encoder<'a> {
    pub fn advance(&mut self, count: usize) {
        self.offset += count;
    }
}

/// The encoded representation of a Rust type
pub trait ComponentType {
    const STRIDE: usize;
}

/// Converts a Rust type into the ABI representation
pub trait Lower: ComponentType {
    /// Lower a Rust type into a memory buffer used to transfer a heap allocated container
    fn lower(&self, cx: &LowerContext, encoder: &mut Encoder<'_>);
    /// Lower a Rust type into a single JsValue to return
    fn lower_ret(&self, cx: &LowerContext) -> JsValue;
    /// Lower a Rust type into a list of arguments to call a javascript function
    fn lower_args(&self, cx: &LowerContext, dst: &mut Array);
}

impl ComponentType for u32 {
    const STRIDE: usize = 4;
}

impl Lower for u32 {
    fn lower(&self, _cx: &LowerContext, encoder: &mut Encoder<'_>) {
        encoder.dv.set_uint32(encoder.offset, *self);
    }

    fn lower_ret(&self, _: &LowerContext) -> JsValue {
        JsValue::from(*self)
    }

    fn lower_args(&self, _cx: &LowerContext, dst: &mut Array) {
        dst.push(&JsValue::from(*self));
    }
}

impl<T: ComponentType> ComponentType for [T] {
    const STRIDE: usize = T::STRIDE;
}

impl<T: Lower + ComponentType> Lower for [T] {
    fn lower(&self, cx: &LowerContext, encoder: &mut Encoder<'_>) {
        // let buffer = Uint8Array::new_with_byte_offset_and_length()
        // let dv = DataView::new(buffer, 0, 0);
    }

    fn lower_ret(&self, cx: &LowerContext) -> JsValue {
        todo!()
    }

    fn lower_args(&self, cx: &LowerContext, dst: &mut Array) {
        todo!()
    }
}

impl<A: ComponentType, B: ComponentType> ComponentType for (A, B) {
    const STRIDE: usize = A::STRIDE + B::STRIDE;
}

impl<A: Lower, B: Lower> Lower for (A, B) {
    fn lower(&self, cx: &LowerContext, encoder: &mut Encoder<'_>) {
        self.0.lower(cx, encoder);
        encoder.offset += A::STRIDE;
        self.1.lower(cx, encoder);
        // For good measure
        encoder.offset += B::STRIDE;
    }

    fn lower_ret(&self, cx: &LowerContext) -> JsValue {
        [self.0.lower_ret(cx), self.1.lower_ret(cx)]
            .iter()
            .collect::<Array>()
            .into()
    }

    fn lower_args(&self, cx: &LowerContext, dst: &mut Array) {
        self.0.lower_args(cx, dst);
        self.1.lower_args(cx, dst);
    }
}

impl<T: ComponentType> ComponentType for (T,) {
    const STRIDE: usize = T::STRIDE;
}

impl<T: Lower> Lower for (T,) {
    fn lower(&self, cx: &LowerContext, encoder: &mut Encoder<'_>) {
        self.0.lower(cx, encoder);
        encoder.offset += T::STRIDE;
    }

    fn lower_ret(&self, cx: &LowerContext) -> JsValue {
        [self.0.lower_ret(cx)].iter().collect::<Array>().into()
    }

    fn lower_args(&self, cx: &LowerContext, dst: &mut Array) {
        self.0.lower_args(cx, dst);
    }
}

/// Converts the ABI representation into a Rust type
pub trait Lift {
    fn lift(value: &JsValue) -> Self;
}
