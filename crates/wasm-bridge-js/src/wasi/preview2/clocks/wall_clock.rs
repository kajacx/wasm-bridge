use std::time::Duration;

use wasm_bridge_macros::{LowerJs, SizeDescription};

use crate::{component::Linker, wasi::preview2::WasiView};
use crate::{Result, StoreContextMut};

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
    linker.instance("wasi:clocks/wall-clock")?.func_wrap(
        "resolution",
        |data: StoreContextMut<T>, (): ()| {
            let clock = data.ctx().wall_clock();
            Ok(clock.resolution().as_nanos() as u64)
        },
    )?;

    linker.instance("wasi:clocks/wall-clock")?.func_wrap(
        "now",
        |data: StoreContextMut<T>, (): ()| {
            let now = data.ctx().wall_clock().now();
            Ok(WallTime {
                seconds: now.as_secs(),
                nanoseconds: now.subsec_nanos(),
            })
        },
    )?;

    Ok(())
}

// TODO: this is kind of hacky ...
mod wasm_bridge {
    pub use crate::*;
}

#[derive(SizeDescription, LowerJs)]
struct WallTime {
    seconds: u64,
    nanoseconds: u32,
}
