use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "records",
});

struct Host;
impl RecordsImports for Host {
    fn move_player(&mut self, mut player: Player, delta: f32) -> Result<Player> {
        player.position.x += player.velocity.x * delta;
        player.position.y += player.velocity.y * delta;
        player.position.z += player.velocity.z * delta;
        Ok(player)
    }

    fn group_import(&mut self, group: Group) -> Result<Group> {
        Ok(group)
    }

    fn increment_single(&mut self, mut single: Single) -> Result<Single> {
        single.value += 1;
        Ok(single)
    }
}

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config).unwrap();
    let mut store = Store::new(&engine, Host);

    let component = Component::new(&store.engine(), &component_bytes).unwrap();

    let mut linker = Linker::new(store.engine());
    Records::add_to_linker(&mut linker, |data| data).unwrap();

    let (instance, _) = Records::instantiate(&mut store, &component, &linker).unwrap();

    let player = Player {
        name: "Mike".into(),
        health: 80,
        position: Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        velocity: Vector {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        tag: 10,
    };

    super::bench("move players", || {
        let result = instance
            .call_move_players(&mut store, &[player.clone(), player.clone()], 2.0)
            .unwrap();
        assert_eq!(
            result[0].position,
            Vector {
                x: 3.0,
                y: 4.0,
                z: 5.0,
            }
        );
    });

    super::bench("increment single", || {
        let result = instance
            .call_increment_single_times(&mut store, Single { value: 5 }, 2)
            .unwrap();
        assert_eq!(result, Single { value: 7 });
    });

    test_wrappers();

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn test_wrappers() {
    let five: wasm_bridge::wasm_bindgen::JsValue = 5.into();
    let three: wasm_bridge::wasm_bindgen::JsValue = 3.into();

    let native: wasm_bridge::js_sys::Function = wasm_bridge::js_sys::eval("(a, b) => a + b")
        .expect("eval native")
        .into();

    super::bench("native", || {
        native
            .call2(
                &wasm_bridge::wasm_bindgen::JsValue::UNDEFINED,
                &five,
                &three,
            )
            .expect("call native")
    });

    let wrapping: wasm_bridge::js_sys::Function =
        wasm_bridge::js_sys::eval("inner => (a, b) => inner(a, b)")
            .expect("eval wrapping")
            .into();
    let wrapping: wasm_bridge::js_sys::Function = wrapping
        .call1(&wasm_bridge::wasm_bindgen::JsValue::UNDEFINED, &native)
        .expect("wrap fn")
        .into();

    super::bench("wrapping", || {
        wrapping
            .call2(
                &wasm_bridge::wasm_bindgen::JsValue::UNDEFINED,
                &five,
                &three,
            )
            .expect("call wrapping")
    });

    let expanding: wasm_bridge::js_sys::Function =
        wasm_bridge::js_sys::eval("inner => (...args) => inner(...args)")
            .expect("eval expanding")
            .into();
    let expanding: wasm_bridge::js_sys::Function = expanding
        .call1(&wasm_bridge::wasm_bindgen::JsValue::UNDEFINED, &native)
        .expect("expand fn")
        .into();

    super::bench("expanding", || {
        expanding
            .call2(
                &wasm_bridge::wasm_bindgen::JsValue::UNDEFINED,
                &five,
                &three,
            )
            .expect("call expanding")
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn test_wrappers() {}
