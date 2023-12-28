use std::time::Duration;

use wasm_bridge::component::Linker;
use wasm_bridge::{Result, StoreContextMut};

use crate::preview2::WasiView;

use self::wasi::clocks::wall_clock::WallTime;

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

wasm_bridge::component::bindgen!({
    path: "src/preview2/clocks/wall_clock.wit",
    world: "exports"
});

impl<T: WasiView> wasi::clocks::wall_clock::Host for T {
    fn resolution(&mut self) -> Result<u64> {
        Ok(self.ctx().wall_clock().resolution().as_nanos() as u64)
    }

    fn now(&mut self) -> Result<wasi::clocks::wall_clock::WallTime> {
        let now = self.ctx().wall_clock().now();
        Ok(WallTime {
            seconds: now.as_secs(),
            nanoseconds: now.subsec_nanos(),
        })
    }
}

pub(crate) fn add_to_linker<T: WasiView>(linker: &mut Linker<T>) -> Result<()> {
    Exports::add_to_linker(linker, |d| d)
}

// pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
//     linker.instance("wasi:clocks/wall-clock")?.func_wrap(
//         "resolution",
//         |data: StoreContextMut<T>, (): ()| {
//             let clock = data.ctx().wall_clock();
//             Ok(clock.resolution().as_nanos() as u64)
//         },
//     )?;

//     linker.instance("wasi:clocks/wall-clock")?.func_wrap(
//         "now",
//         |data: StoreContextMut<T>, (): ()| {
//             let now = data.ctx().wall_clock().now();
//             Ok(WallTime {
//                 seconds: now.as_secs(),
//                 nanoseconds: now.subsec_nanos(),
//             })
//         },
//     )?;

//     Ok(())
// }

// #[derive(wasm_bridge_macros::SizeDescription, wasm_bridge_macros::LowerJs)]
// struct WallTime {
//     seconds: u64,
//     nanoseconds: u32,
// }
