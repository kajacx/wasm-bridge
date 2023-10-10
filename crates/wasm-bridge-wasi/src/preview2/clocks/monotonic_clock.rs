use crate::preview2::WasiView;
use wasm_bridge::{component::Linker, Result, StoreContextMut};

pub trait HostMonotonicClock: Send + Sync {
    fn resolution(&self) -> u64;
    fn now(&self) -> u64;
}

struct JsClock;

impl HostMonotonicClock for JsClock {
    fn resolution(&self) -> u64 {
        // in nano seconds, so ...
        // 1_000 // one micro second
        1_000_000 // one millisecond

        // The accuracy seems to be 0.1 milliseconds, but let's say one millisecond to be sure
    }

    fn now(&self) -> u64 {
        // performance gives milliseconds, but we want nano seconds
        (js_sys::eval("performance.now()").unwrap().as_f64().unwrap() * 1_000_000.0) as _
    }
}

pub(crate) fn default_monotonic_clock() -> impl HostMonotonicClock {
    JsClock
}

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    linker.instance("wasi:clocks/monotonic-clock")?.func_wrap(
        "resolution",
        |data: StoreContextMut<T>, (): ()| {
            let clock = data.ctx().monotonic_clock();
            Ok(clock.resolution())
        },
    )?;

    linker.instance("wasi:clocks/monotonic-clock")?.func_wrap(
        "now",
        |data: StoreContextMut<T>, (): ()| {
            let clock = data.ctx().monotonic_clock();
            Ok(clock.now())
        },
    )?;

    Ok(())
}
