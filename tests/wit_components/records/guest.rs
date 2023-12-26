wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "records",
    exports: {
        world: MyGuest,
    }
});

struct MyGuest;

impl Guest for MyGuest {
    fn move_players(mut players: Vec<Player>, delta: f32) -> Vec<Player> {
        players.iter_mut().for_each(|player| {
            *player = move_player(player, delta);
        });
        players
    }

    fn group_export(group: Group) -> Group {
        group
    }
}
