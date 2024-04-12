use std::time::Duration;

use wasm_bridge::component::Linker;
use wasm_bridge::{Result, StoreContextMut};

use crate::js::WasiView;

pub trait HostWallClock: Send + Sync {
    fn resolution(&self) -> Duration;
    fn now(&self) -> Duration;
}

struct JsClock;

impl HostWallClock for JsClock {
    fn resolution(&self) -> Duration {
        Duration::from_millis(1) // Again, say 1 ms
    }

    fn now(&self) -> Duration {
        let millis = js_sys::eval("new Date().getTime()")
            .unwrap()
            .as_f64()
            .unwrap();
        Duration::from_millis(millis as _)
    }
}

pub(crate) fn real_wall_clock() -> impl HostWallClock {
    JsClock
}

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    linker
        .instance("wasi:clocks/wall-clock@0.2.0-rc-2023-11-10")?
        .func_wrap("now", |mut caller: StoreContextMut<T>, ()| {
            let now = caller.data_mut().ctx().wall_clock().now();
            Ok(WallTime {
                seconds: now.as_secs(),
                nanoseconds: now.subsec_nanos(),
            })
        })
}

#[derive(
    wasm_bridge_macros::SizeDescription, wasm_bridge_macros::LowerJs, wasm_bridge_macros::LiftJs,
)]
struct WallTime {
    seconds: u64,
    nanoseconds: u32,
}
