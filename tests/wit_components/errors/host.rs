use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "errors",
});

struct HostData;

impl ErrorsImports for HostData {
    fn simple_fail_host(&mut self, fail: WhereFail) -> Result<WhereFail> {
        match fail {
            // TODO: hacky import ...
            WhereFail::HostErr => Err(wasm_bridge::component::__internal::anyhow::anyhow!(
                "host err"
            )),
            other => Ok(other),
        }
    }

    fn full_fail_host(&mut self, fail: WhereFail) -> Result<Result<WhereFail, WhereFail>> {
        match fail {
            // TODO: hacky import ...
            WhereFail::HostErr => Err(wasm_bridge::component::__internal::anyhow::anyhow!(
                "host err"
            )),
            WhereFail::HostOkErr => Ok(Err(WhereFail::HostOkErr)),
            WhereFail::HostOkOk => Ok(Ok(WhereFail::HostOkOk)),
            _ => unreachable!("guest should not call host with Guest_ variants"),
        }
    }
}

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, HostData);

    let component = Component::new(&store.engine(), component_bytes)?;

    let mut linker = Linker::new(store.engine());
    Errors::add_to_linker(&mut linker, |data| data)?;

    // We need a new instance for every test because of the "cannot reenter component instance" error
    // Kind of annoying, but what are we going to do ...

    // Simple return value
    let (instance, _) = Errors::instantiate(&mut store, &component, &linker)?;
    let result = instance.call_simple_fail_guest(&mut store, WhereFail::HostOkOk)?;
    assert_eq!(result, WhereFail::HostOkOk);

    let (instance, _) = Errors::instantiate(&mut store, &component, &linker)?;
    instance
        .call_simple_fail_guest(&mut store, WhereFail::GuestPanic)
        .expect_err("guest code should panic");

    let (instance, _) = Errors::instantiate(&mut store, &component, &linker)?;
    instance
        .call_simple_fail_guest(&mut store, WhereFail::HostErr)
        .expect_err("host code should return err");

    // Full return value
    let (instance, _) = Errors::instantiate(&mut store, &component, &linker)?;
    instance
        .call_full_fail_guest(&mut store, WhereFail::GuestPanic)
        .expect_err("guest code should panic");

    let (instance, _) = Errors::instantiate(&mut store, &component, &linker)?;
    let result = instance.call_full_fail_guest(&mut store, WhereFail::GuestErr)?;
    assert_eq!(result, Err(WhereFail::GuestErr));

    let (instance, _) = Errors::instantiate(&mut store, &component, &linker)?;
    instance
        .call_full_fail_guest(&mut store, WhereFail::HostErr)
        .expect_err("host code should return err");

    let (instance, _) = Errors::instantiate(&mut store, &component, &linker)?;
    let result = instance.call_full_fail_guest(&mut store, WhereFail::HostOkErr)?;
    assert_eq!(result, Err(WhereFail::HostOkErr));

    let (instance, _) = Errors::instantiate(&mut store, &component, &linker)?;
    let result = instance.call_full_fail_guest(&mut store, WhereFail::HostOkOk)?;
    assert_eq!(result, Ok(WhereFail::HostOkOk));

    Ok(())
}
