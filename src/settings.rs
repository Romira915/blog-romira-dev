use std::path::PathBuf;

use once_cell::sync::Lazy;
use serde::Deserialize;

const NEWT_CDN_BASE_URL: &'static str = "https://blog-romira-dev.cdn.newt.so";

#[derive(Debug, Deserialize)]
pub struct Config {
    pub app_origin: String,
    pub rust_log: String,
    pub newt_api_key: String,
}

impl Config {
    pub fn rust_log(&self) -> &str {
        &self.rust_log
    }

    pub fn newt_api_key(&self) -> &str {
        &self.newt_api_key
    }

    pub fn rust_log_to_level_filter(&self) -> log::LevelFilter {
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

// pub fn init_logger(
//     log_level: log::LevelFilter,
//     log_file_path: Option<PathBuf>,
// ) -> Result<(), fern::InitError> {
//     let colors = ColoredLevelConfig::new()
//         .info(Color::Green)
//         .trace(Color::BrightBlack);

//     let base_config = fern::Dispatch::new()
//         .level(log_level)
//         .level_for("axum", log::LevelFilter::Info)
//         .level_for("hyper", log::LevelFilter::Info)
//         .level_for("tower-http", log::LevelFilter::Info)
//         .level_for("tower", log::LevelFilter::Info);

//     let stdout_config = fern::Dispatch::new()
//         .format(move |out, message, record| {
//             out.finish(format_args!(
//                 "{}[{}][{}] {}",
//                 chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
//                 record.target(),
//                 colors.color(record.level()),
//                 message
//             ))
//         })
//         .chain(std::io::stdout());

//     let file_config = if let Some(log_file_path) = log_file_path {
//         fern::Dispatch::new()
//             .format(|out, message, record| {
//                 out.finish(format_args!(
//                     "{}[{}][{}] {}",
//                     chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
//                     record.target(),
//                     record.level(),
//                     message
//                 ))
//             })
//             .chain(fern::log_file(log_file_path)?)
//     } else {
//         fern::Dispatch::new()
//     };

//     base_config
//         .chain(stdout_config)
//         .chain(file_config)
//         .apply()?;

//     Ok(())
// }
