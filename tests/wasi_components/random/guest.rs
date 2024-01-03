use rand::Rng;

wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "random",
    exports: {
        world: GuestImpl
    }
});

struct GuestImpl;

impl Guest for GuestImpl {
    fn random_number() -> u64 {
        rand::thread_rng().gen::<u64>()
    }

    fn random_bytes() -> Vec<u8> {
        rand::thread_rng().gen::<[u8; 32]>().into()
    }

    //     fn round_trip_export(l: Vec<u32>) -> Vec<u32> {
    //         round_trip_import(&l)
    //     }
}
