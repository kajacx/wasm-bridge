use std::marker::PhantomData;

pub struct Instance {
    exports: JsValue,
}

impl Instance {
    fn new(exports: JsValue) -> Self {
        Self { exports }
    }
}

// pub struct InstancePre<T> {
//     _phantom: PhantomData<T>,
// }
