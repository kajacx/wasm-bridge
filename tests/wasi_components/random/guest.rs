use rand::Rng;

wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "random",
});

struct GuestImpl;

impl Random for GuestImpl {
    fn random_number() -> u64 {
        rand::random::<u64>()
    }

    fn random_bytes() -> Vec<u8> {
        rand::thread_rng().gen::<[u8; 32]>().into()
    }
}

export_random!(GuestImpl);
