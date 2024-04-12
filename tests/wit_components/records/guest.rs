wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "records",
});

struct GuestImpl;

impl Guest for GuestImpl {
    fn move_players(mut players: Vec<Player>, delta: f32) -> Vec<Player> {
        players.iter_mut().for_each(|player| {
            *player = move_player(player, delta);
        });
        players
    }

    fn group_export(group: Group) -> Group {
        group_import(&group)
    }

    fn increment_single_times(mut single: Single, times: u32) -> Single {
        for _ in 0..times {
            single = increment_single(single);
        }
        single
    }

    fn use_multiword(_m: Multiword) -> MultiWord {
        todo!()
    }
}

export!(GuestImpl);
