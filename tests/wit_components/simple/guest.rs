wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "simple",
    exports: {
        world: MyGuest,
    }
});

struct MyGuest;

impl Guest for MyGuest {
    fn push_s32s(mut numbers: Vec<i32>, a: i32, b: i32) -> Vec<i32> {
        numbers.push(a);
        numbers.push(b);
        numbers
    }

    fn push_u32s(mut numbers: Vec<u32>, a: u32, b: u32) -> Vec<u32> {
        numbers.push(a);
        numbers.push(b);
        numbers
    }

    fn voider() {}

    fn pairs12() -> (i32, i32) {
        (1, 2)
    }

    fn get_vector123() -> Vector {
        Vector { x: 1, y: 2, z: 3 }
    }

    fn get_many_vectors(amount: u32) -> Vec<Vector> {
        (0..amount as usize)
            .map(|i| Vector {
                x: i as _,
                y: i as _,
                z: i as _,
            })
            .collect()
    }

    fn heal_players(mut players: Vec<Player>, healing: u32) -> Vec<Player> {
        players.iter_mut().for_each(|player| {
            player.health = u32::min(player.health + healing, player.max_health)
        });
        players
    }

    fn move_players(mut players: Vec<Player>, scale: i32) -> Vec<Player> {
        players.iter_mut().for_each(|player| {
            player.position.x += player.velocity.x * scale;
            player.position.y += player.velocity.y * scale;
            player.position.z += player.velocity.z * scale;
        });
        players
    }

    fn player_look_at(mut player: Player, look: Vector) -> Player {
        player.looking_at = look;
        player
    }
}
