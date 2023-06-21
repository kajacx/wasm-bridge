const PLUGIN_WAT: &'static [u8] = include_bytes!("../../plugin/plugin.wat");

mod host;

fn main() {
    host::run_test(PLUGIN_WAT).expect("host_sys test should pass")
}
