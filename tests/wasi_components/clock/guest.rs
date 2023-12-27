wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "clock",
    exports: {
        world: GuestImpl,
    }
});

struct GuestImpl;

impl Guest for GuestImpl {
    fn seconds_since_epoch() -> u64 {
        let now = std::time::SystemTime::now();
        let interval = now
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap();
        interval.as_secs()
    }

    fn nanoseconds_bench() -> u64 {
        let now = std::time::Instant::now();

        // some random computation
        let mut result = 0;
        for i in 0..1_000_000 {
            result += i % 4;
        }

        let elapsed = now.elapsed().as_nanos() as u64;

        // Make sure the result "contributes" to the return value,
        // so that it isn't optimized, but also make sure it is 0.
        let result = (result as f64 / 1.0e-15) as u64;

        elapsed + result
    }
}
