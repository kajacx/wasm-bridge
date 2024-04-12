wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "environment",
});

struct GuestImpl;

impl Guest for GuestImpl {
    fn get_env_var(name: String) -> Option<String> {
        std::env::var(name).ok()
    }
}

export!(GuestImpl);
