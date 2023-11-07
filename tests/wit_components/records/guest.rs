wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "records",
});

stuct Guest;

impl Records for Guest {
    fn get_inventory(player: Player) -> Vec<u32> {
        player.inventory
    }
}

export_records!(Guest);
