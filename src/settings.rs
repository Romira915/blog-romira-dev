use std::path::PathBuf;

use fern::colors::{Color, ColoredLevelConfig};
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app_origin: String,
    pub trace_level: String,
    pub rust_log: String,
}

impl Config {
    pub fn rust_log(&self) -> log::LevelFilter {
        match self.rust_log.to_lowercase().as_str() {
            "error" => log::LevelFilter::Error,
            "warn" => log::LevelFilter::Warn,
            "info" => log::LevelFilter::Info,
            "debug" => log::LevelFilter::Debug,
            "trace" => log::LevelFilter::Trace,
            _ => log::LevelFilter::Off,
        }
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv::dotenv().unwrap_or_default();
    envy::from_env().expect("Failed to env to Config.")
});

pub fn init_logger(
    log_level: log::LevelFilter,
    log_file_path: Option<PathBuf>,
) -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new().info(Color::Green);

    let config = fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        // Add blanket level filter -
        .level(log_level)
        // - and per-module overrides
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout());

    if let Some(log_file_path) = log_file_path {
        config.chain(fern::log_file(log_file_path)?)
    } else {
        config
    }
    .apply()?;

    Ok(())
}
