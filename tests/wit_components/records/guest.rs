wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "records",
    exports: {
        world: MyGuest,
    }
});

struct MyGuest;

impl Guest for MyGuest {
    fn get_inventory(player: Player) -> Vec<u32> {
        player.inventory
    }
}
