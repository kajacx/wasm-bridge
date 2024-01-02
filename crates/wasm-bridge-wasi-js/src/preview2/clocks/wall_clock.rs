use std::time::Duration;

use wasm_bridge::component::Linker;
use wasm_bridge::{Result, StoreContextMut};

use crate::preview2::WasiView;

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

// wasm_bridge::component::bindgen!({
//     path: "src/preview2/wits/wall_clock.wit",
//     world: "exports"
// });

// impl<T: WasiView> ExportsImports for T {
//     // fn resolution(&mut self) -> Result<u64> {
//     //     Ok(self.ctx().wall_clock().resolution().as_nanos() as u64)
//     // }

//     fn clock_time_get(&mut self) -> Result<WallTime> {
//         panic!("IT IS GETTING CALLED?");
//         let now = self.ctx().wall_clock().now();
//         Ok(WallTime {
//             seconds: now.as_secs(),
//             nanoseconds: now.subsec_nanos(),
//         })
//     }
// }

// pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
//     Exports::add_to_linker(linker, |d| d)
// }

pub(crate) fn add_to_linker<T: WasiView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    linker
        .instance_wasi("wasi:clocks/wall-clock@0.2.0-rc-2023-11-10")?
        .func_wrap("now", |caller: StoreContextMut<T>, ()| {
            let now = caller.data().ctx().wall_clock().now();
            Ok(WallTime {
                seconds: now.as_secs(),
                nanoseconds: now.subsec_nanos(),
            })
        })
    // linker.instance("wasi:clocks/wall-clock")?.func_wrap(
    //     "resolution",
    //     |data: StoreContextMut<T>, (): ()| {
    //         let clock = data.ctx().wall_clock();
    //         Ok(clock.resolution().as_nanos() as u64)
    //     },
    // )?;

    // linker.instance("wasi_snapshot_preview1")?.func_wrap(
    //     "clock_time_get",
    //     |data: StoreContextMut<T>, (): ()| {
    //         // panic!("IT IS GETTING CALLED NOWWW?");
    //         let now = data.ctx().wall_clock().now();
    //         // Ok(WallTime {
    //         //     seconds: now.as_secs(),
    //         //     nanoseconds: now.subsec_nanos(),
    //         // })
    //         Ok(now.as_secs() as u32)
    //     },
    // )?;

    // Ok(())
}

#[derive(
    wasm_bridge_macros::SizeDescription, wasm_bridge_macros::LowerJs, wasm_bridge_macros::LiftJs,
)]
struct WallTime {
    seconds: u64,
    nanoseconds: u32,
}
