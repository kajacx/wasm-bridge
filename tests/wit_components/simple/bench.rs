use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "simple",
});

pub fn run_test(component_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, ());

    let component = Component::new(&store.engine(), &component_bytes)?;

    let linker = Linker::new(store.engine());

    let (instance, _) = Simple::instantiate(&mut store, &component, &linker)?;

    let players = (0..50)
        .map(|i| {
            let i = i as i32;
            Player {
                position: Vector {
                    x: i,
                    y: i + 1,
                    z: i + 2,
                },
                velocity: Vector {
                    x: i,
                    y: i - 1,
                    z: i - 2,
                },
                looking_at: Vector {
                    x: i,
                    y: i + 1,
                    z: i - 1,
                },
                health: (i as u32 % 10) + 10,
                max_health: 20,
            }
        })
        .collect::<Vec<_>>();

    super::bench("Manipulate with many players", || {
        let result = instance.call_heal_players(&mut store, &players, 5).unwrap();
        assert_eq!(result[13].health, 18);
        assert_eq!(result[18].health, 20);

        let result = instance.call_move_players(&mut store, &players, 2).unwrap();
        assert_eq!(result[2].position.x, 6);
        assert_eq!(result[3].position.z, 7);
    });

    Ok(())
}
