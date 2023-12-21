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

    let big_vec: Vec<_> = (0..1000).into_iter().collect();

    super::bench("Call exported methods", || {
        let result = instance.call_push_s32s(&mut store, &big_vec, 3, 4).unwrap();
        assert_eq!(result.len(), 1002);

        let result = instance
            .call_push_u32s(&mut store, &[10, u32::MAX - 10], 3, 4)
            .unwrap();
        assert_eq!(result, vec![10, u32::MAX - 10, 3, 4]);
    });

    instance.call_voider(&mut store).expect("call voider");

    let result = instance.call_pairs12(&mut store)?;
    assert_eq!(result, (1, 2));

    super::bench("Call get single vector", || {
        let result = instance.call_get_vector123(&mut store).unwrap();
        assert_eq!((result.x, result.y, result.z), (1, 2, 3));
    });

    super::bench("Call get many vectors", || {
        let amount = 200;
        let result = instance.call_get_many_vectors(&mut store, amount).unwrap();
        assert_eq!(result.len(), amount as usize);
    });

    let players = (0..100)
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

    let pos = Vector { x: 1, y: 2, z: 3 };
    let look = Vector { x: 4, y: 5, z: 6 };
    let player = Player {
        position: pos.clone(),
        velocity: pos.clone(),
        looking_at: pos,
        health: 15,
        max_health: 20,
    };

    super::bench("Pass around a single player", || {
        let result = instance
            .call_player_look_at(&mut store, player, look)
            .unwrap();
        assert_eq!(result.looking_at.z, 6);
    });

    Ok(())
}
