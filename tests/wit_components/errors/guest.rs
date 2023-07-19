wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "errors",
});

struct Guest;

impl Errors for Guest {
    fn simple_fail_guest(fail: WhereFail) -> WhereFail {
        match fail {
            WhereFail::GuestPanic => panic!("Fail in guest code with panic"),
            other => simple_fail_host(other),
        }
    }

    fn full_fail_guest(fail: WhereFail) -> Result<WhereFail, WhereFail> {
        match fail {
            WhereFail::GuestPanic => panic!("Fail in guest code with panic"),
            WhereFail::GuestErr => Err(WhereFail::GuestErr),
            other => full_fail_host(other),
        }
    }
}

export_errors!(Guest);
