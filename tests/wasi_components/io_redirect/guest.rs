wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "io-redirect",
});

struct GuestImpl;

impl IoRedirect for GuestImpl {
    fn readln_from_stdin() -> Option<String> {
        None // TODO:
    }

    fn writeln_to_stdout(line: String) {
        println!("{line}");
    }

    fn writeln_to_stderr(line: String) {
        eprintln!("{line}");
    }
}

export_io_redirect!(GuestImpl);
