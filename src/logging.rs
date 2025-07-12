use tracing::{subscriber::set_global_default, Level};
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::format::{Compact, DefaultFields, Format},
    prelude::*,
    reload::{self, Handle},
    FmtSubscriber,
};

type ReloadHandle = Handle<LevelFilter, FmtSubscriber<DefaultFields, Format<Compact, ()>>>;

/// Init logger with provided level
pub fn init_logger(initial_level: Level) -> ReloadHandle {
    let filter = LevelFilter::from_level(initial_level);
    let (filter_layer, reload_handle) = reload::Layer::new(filter);

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .compact()
        .without_time()
        .finish()
        .with(filter_layer);

    set_global_default(subscriber).expect("Failed to set global default subscriber");

    reload_handle
}

/// Set logging level for existing logger
pub fn set_log_level(handle: ReloadHandle, level: tracing::Level) {
    handle
        .modify(|filter| *filter = LevelFilter::from_level(level))
        .expect("Failed to change log level");
}

/// Get log level from string
pub fn str_to_log_level(level: &str) -> Option<Level> {
    match level.to_lowercase().as_str() {
        "trace" => Some(Level::TRACE),
        "debug" => Some(Level::DEBUG),
        "info" => Some(Level::INFO),
        "warn" => Some(Level::WARN),
        "error" => Some(Level::ERROR),
        _ => None,
    }
}
