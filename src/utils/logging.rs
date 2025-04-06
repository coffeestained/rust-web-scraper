use std::env;
use env_logger::Builder;
use log::LevelFilter;

pub use log::{info, debug, warn, error, trace};

pub fn init() {
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let level_filter = match log_level.to_lowercase().as_str() {
        "debug" => LevelFilter::Debug,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "trace" => LevelFilter::Trace,
        "off" => LevelFilter::Off,
        _ => LevelFilter::Info,
    };

    Builder::new()
        .filter_level(level_filter)
        .format_timestamp_secs()
        .init();
}