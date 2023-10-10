pub fn setup_tracing() {
    use tracing_subscriber::prelude::*;
    #[cfg(target_arch = "wasm32")]
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true) // Only partially supported across browsers
        .without_time()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .with_writer(tracing_web::MakeConsoleWriter); // write events to the console

    #[cfg(not(target_arch = "wasm32"))]
    let fmt_layer = tracing_subscriber::fmt::layer().with_ansi(true);

    tracing_subscriber::registry().with(fmt_layer).init();
}
