use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use csv::ReaderBuilder;
use std::error::Error;
use std::fs::File;
use walkdir::WalkDir;
use std::collections::HashMap;

//use std::path::PathBuf;

/// Copies data from source to destination without overwriting existing files.
fn copy_data_dir(src: &Path, dest: &Path) -> std::io::Result<()> {
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let relative_path = entry.path().strip_prefix(src).unwrap();
        let dest_path = dest.join(relative_path);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else if !dest_path.exists() {
            fs::copy(entry.path(), &dest_path)?;
        }
    }
    Ok(())
}

/// this returns the datadir based on project mode...
pub fn read_csv(file_path: PathBuf) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let mut records: Vec<Vec<String>> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        records.push(record.iter().map(|s| s.to_string()).collect()); // Convert to Vec<String>
    }

    Ok(records)
}





/// Returns the correct data directory for the app.
pub fn get_data_dir() -> PathBuf {
    // Get the path of the running executable
    let exe_path = env::current_exe().expect("Failed to get executable path");

    // Move two levels up to get to the project root (from target/debug/yourapp)
    let project_root = exe_path.ancestors().nth(3).expect("Failed to get project root");

    #[cfg(debug_assertions)]
    {
        // In development, use <project_root>/data/
        return project_root.join("data");
    }

    // In release mode, use ~/.appname-data
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let app_name = env!("CARGO_PKG_NAME");
    PathBuf::from(format!("{}/.{}-data", home_dir, app_name))
}



// pub fn get_data_dir() -> PathBuf {
//     let app_name = env!("CARGO_PKG_NAME");

//     #[cfg(debug_assertions)]
//     {
//         // Use local data directory in development mode
//         return PathBuf::from(format!("{}/data", env!("CARGO_MANIFEST_DIR")));
//     }

//     // In release mode, use system locations
//     PathBuf::from(format!("/usr/share/{}", app_name))
// }


/// Reads config.cfg and returns a HashMap of key-value pairs.
pub fn read_config() -> HashMap<String, String> {
    let config_file = get_data_dir().join("config.cfg");
//    let mut config_map =  std::collections::HashMap::new();
    let mut config_map =  HashMap::new();

    if let Ok(content) = fs::read_to_string(config_file) {
        for line in content.lines() {

            if let Some((key, value)) = line.split_once('=') {
                let clean_key = key.trim().to_string();
                let clean_value = value.trim().trim_matches('"').to_string();
                config_map.insert(clean_key, clean_value);
            }
        }
    }

    config_map
}

