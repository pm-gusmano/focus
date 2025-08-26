use std::{fs, io, path::PathBuf};
use toml;

// Helper: Find config directory
pub fn find_config_dir() -> io::Result<PathBuf> {
    let proj_dirs = directories::ProjectDirs::from("com", "chetanxpro", "focusguard")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "ProjectDirs not found"))?;
    Ok(proj_dirs.config_dir().to_path_buf())
}

// Helper: Ensure config.toml exists
pub fn ensure_config_file_exists(config_dir: &PathBuf) -> io::Result<PathBuf> {
    let toml_config_path = config_dir.join("config.toml");
    if !toml_config_path.exists() {
        println!("Please run `focus setup --list <exact path to website list>` to setup focus");
        std::process::exit(1);
    }
    Ok(toml_config_path)
}

// Helper: Ensure config directory exists
pub fn ensure_dir_exists(config_dir: &PathBuf) -> io::Result<()> {
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).expect("Error while creating config directory");
    }
    Ok(())
}

/// Returns the path to the config.toml file, ensuring it exists and instructing the user if missing.
pub fn get_toml_config_path() -> io::Result<PathBuf> {
    let config_dir = find_config_dir()?;
    ensure_dir_exists(&config_dir)?;
    ensure_config_file_exists(&config_dir)
}

/// Reads config.toml made from the `setup` command and returns the websites
/// file path, or an error if not set.
pub fn get_string_from_config(config_path: PathBuf) -> io::Result<String> {
    let config_content = fs::read_to_string(config_path)?;
    let config_toml_data: toml::Value = toml::from_str(&config_content)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid TOML in config file"))?;
    match config_toml_data
        .get("website_list_path")
        .and_then(toml::Value::as_str)
    {
        Some(path) => Ok(path.to_string()),
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "website_list_path not set in config",
        )),
    }
}
