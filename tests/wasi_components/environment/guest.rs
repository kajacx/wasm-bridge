wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "environment",
    exports: {
        world: GuestImpl,
    }
});

struct GuestImpl;

impl Guest for GuestImpl {
    fn get_env_var(name: String) -> Option<String> {
        std::env::var(name).ok()
    }
}
