wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "clock",
});

struct GuestImpl;

impl Clock for GuestImpl {
    fn seconds_since_epoch() -> u64 {
        let now = std::time::SystemTime::now();
        let interval = now
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap();
        interval.as_secs()
    }
}

export_clock!(GuestImpl);
