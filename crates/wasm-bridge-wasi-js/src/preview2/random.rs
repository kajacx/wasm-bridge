use wasm_bridge::component::Linker;
use wasm_bridge::{Result, StoreContextMut};

use super::WasiView;

struct MathRandom;

impl rand_core::RngCore for MathRandom {
    fn next_u32(&mut self) -> u32 {
        let value = js_sys::eval("Math.random()").expect("eval math random");
        let value = value.as_f64().expect("math random should be a number");

        let value = value * u32::MAX as f64;
        value as u32
    }

    fn next_u64(&mut self) -> u64 {
        let first = self.next_u32();
        let second = self.next_u32();
        (first as u64) << 32 | second as u64
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

pub(crate) type SecureRandom = Box<dyn rand_core::RngCore + Send + Sync>;

pub(crate) fn js_rand() -> SecureRandom {
    // TODO: not actually secure
    Box::new(MathRandom)
}

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    linker
        .instance("wasi:random/random@0.2.0-rc-2023-11-10")?
        .func_wrap(
            "get-random-bytes",
            |mut caller: StoreContextMut<T>, (len,): (u64,)| {
                let mut bytes = vec![0u8; len as usize];
                caller.data_mut().ctx_mut().random().fill_bytes(&mut bytes);
                Ok(bytes)
            },
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[wasm_bindgen_test::wasm_bindgen_test]
    fn test_generator() {
        let mut rng = js_rand();

        let numbers: Vec<_> = (0..100).map(|_| rng.next_u32()).collect();

        // TODO: these tests are not great. There is about 1 : 40 000 chance they will randomly fail
        assert!(
            *numbers.iter().min().unwrap() < u32::MAX / 10,
            "At least one number should be smaller than 10%"
        );

        assert!(
            *numbers.iter().max().unwrap() > u32::MAX / 10 * 9,
            "At least one number should be bigger than 90%"
        );
    }
}
