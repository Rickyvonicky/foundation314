mod config;
mod logging;

fn main() {
    let config_path = config::get_config_dir();
    logging::log_message(&format!("Config directory is at {:?}", config_path));
}
