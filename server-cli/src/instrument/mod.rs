use tracing_subscriber::{EnvFilter, prelude::*, util::SubscriberInitExt};

static INIT: std::sync::Once = std::sync::Once::new();

pub fn init() {
    INIT.call_once(|| {
        let console_layer = console_subscriber::ConsoleLayer::builder().spawn();
        let fmt_layer = tracing_subscriber::fmt::layer();
        let filter = EnvFilter::new("info");

        tracing_subscriber::registry()
            .with(console_layer)
            .with(fmt_layer)
            .with(filter)
            .try_init()
            .unwrap_or_else(|_| {
                tracing::debug!("tracing already initialized; skipping re-init");
            });
    });
}
