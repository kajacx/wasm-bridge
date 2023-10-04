use std::sync::OnceLock;

wit_bindgen::generate!({
    path: "../../records.wit",
    world: "records",
});

struct Guest;

static PLAYER: OnceLock<Player> = OnceLock::new();

fn make_player() -> Player {
    create_player("Foo", &["sword", "shield", "apple"], &[1, 2, 5])
}

impl Records for Guest {
    fn get_inventory() -> Vec<String> {
        PLAYER.get_or_init(make_player).inventory.clone()
    }

    fn get_counts() -> Vec<u32> {
        PLAYER.get_or_init(make_player).counts.clone()
    }
}

export_records!(Guest);
