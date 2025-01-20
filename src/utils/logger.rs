use std::path::PathBuf;

fn get_config_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("config");
    path.push("log4rs.yaml");
    path
}

pub fn init_logger() {
    let config_path = get_config_path();
    log4rs::init_file(&config_path, Default::default()).expect("Failed to initialize logger");
    log::info!("Logger initialized");
    log::debug!(target: "debug", "Debug logger initialized");
    log::error!(target: "error", "Error logger initialized");
}

#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        log::debug!(target: "debug", $($arg)*);
    }
}

#[macro_export]
macro_rules! error_log {
    ($($arg:tt)*) => {
        log::error!(target: "error", $($arg)*)
    }
}
