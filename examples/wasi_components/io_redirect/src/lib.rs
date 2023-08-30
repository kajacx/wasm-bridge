wit_bindgen::generate!({
    path: "../io_redirect.wit",
    world: "io-redirect",
});

struct GuestImpl;

impl IoRedirect for GuestImpl {
    fn readln_from_stdin() -> Option<String> {
        std::io::stdin()
            .lines()
            .next()
            .map(|line| line.expect("read line"))
    }

    fn writeln_to_stdout(line: String) {
        println!("{line}");
    }

    fn writeln_to_stderr(line: String) {
        eprintln!("{line}");
    }
}

export_io_redirect!(GuestImpl);
