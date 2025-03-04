use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;

/// Returns the path to the GameQuest config directory, creating it if needed.
pub fn get_config_dir() -> PathBuf {
    let home_dir = env::var("HOME").expect("Could not find home directory");
    let config_dir = Path::new(&home_dir).join(".gamequesttest");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    }

    config_dir
}

pub fn read_csv(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let mut records: Vec<Vec<String>> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        records.push(record.iter().map(|s| s.to_string()).collect()); // Convert to Vec<String>
    }

    Ok(records)
}
