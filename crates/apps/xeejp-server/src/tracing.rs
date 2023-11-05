use tracing_subscriber::{
    fmt::{format::Pretty, time::UtcTime},
    prelude::__tracing_subscriber_SubscriberExt,
    util::SubscriberInitExt,
};
use tracing_web::{performance_layer, MakeConsoleWriter};

pub fn init_tracing_once() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_ansi(false) // Only partially supported across JavaScript runtimes
        .with_timer(UtcTime::rfc_3339()) // std::time is not available in browsers
        .with_writer(MakeConsoleWriter); // write events to the console
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    let _ = tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .try_init();
}
