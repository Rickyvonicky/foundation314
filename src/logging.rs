// pub fn log_message(msg: &str) {
//     println!("[LOG]: {}", msg);
// }

use std::fs::{OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use chrono::Local;

use crate::config::get_data_dir;

/// Writes a log message to the app log file.
pub fn log_message(message: &str, enable_logging: bool) {
    if !enable_logging {
        return;
    }

    let log_file = get_data_dir().join("app.log");
    let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_file) {
        let _ = writeln!(file, "{} {}", timestamp, message);
    }
}
