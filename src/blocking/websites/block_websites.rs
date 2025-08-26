use directories;

use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use toml::Value;

use crate::{blocking::methods::block_duration, os_backend};

use crate::blocking::ui::spinners::show_interruptible_spinner_for_duration;

// Make a struct in the blocking module that describes exactly what is to be
// blocked (websites first, then apps later; how long, and a short-hand name
// for that combination of what's to be blocked and for how long. The programs
// that are blocked, the conditions for their blocking (at this point, just a
// duration of time, but later more, like a schedule or location,) and the name
// for that combination can be stored in a yaml or toml file in a few config
// directories, and represented by a struct in Rust.

pub fn block_websites_via_hosts_config_change(
    user_input_time: &String,
    task: Option<&String>,
) -> io::Result<()> {
    // Get host path from os_backend
    let hosts_path = os_backend::get_hosts_path();

    // Using the config file from the `setup` CLI command, get the list of websites to block
    let blocked_websites_list = get_blocked_website_list_from_toml_config()?;

    // Prepare a backup of the hosts file and ensure they exist
    let backup_path = prepare_hosts_backups()?;
    fs::copy(&hosts_path, &backup_path)?;

    let hosts_content = fs::read_to_string(&hosts_path)?;
    let hosts_file_with_blocked_websites =
        rewrite_hosts_contents_to_block_websites(&hosts_content, &blocked_websites_list);
    println!("Content:\n {}", hosts_file_with_blocked_websites);

    fs::write(&hosts_path, &hosts_file_with_blocked_websites)?;

    // Setup, then display an interruptible timer
    let formatted_message = generate_blocking_message(user_input_time, task);
    let duration_to_wait = block_duration::parse_time_string(&user_input_time);
    show_interruptible_spinner_for_duration(&duration_to_wait, &formatted_message)?;

    // After the timer ends or is exited early, restore the hosts file
    restore_hosts_file(&backup_path, &hosts_path)?;

    // Inform the user
    println!("\n  Unblocked websites ✅");
    Ok(())
}

fn get_blocked_website_list_from_toml_config() -> Result<String, io::Error> {
    // Get the path to the config file
    let toml_config_path = get_toml_config_path()?;

    // Read the config file to generate the file path to the websites list
    let blocked_websites_file_path = get_websites_file_path_from_config(toml_config_path)?;

    // Get the list of blocked websites
    let blocked_websites_list = fs::read_to_string(blocked_websites_file_path)?;
    Ok(blocked_websites_list)
}

fn generate_blocking_message(user_input_time: &String, task: Option<&String>) -> String {
    // Make a message to inform the user what's being blocked.
    let formatted_message = match task {
        Some(t) => format!("Blocked websites for {} for task: {}", user_input_time, t),
        None => format!("Blocked websites for {}", user_input_time),
    };
    formatted_message
}

// Helper: Setup config paths
/// Returns the paths for the hosts backup and config.toml file.
/// Ensures the config directory and backup files exist, and instructs the user if setup is missing.

// Helper: Find config directory
fn find_config_dir() -> io::Result<PathBuf> {
    let proj_dirs = directories::ProjectDirs::from("com", "chetanxpro", "focusguard")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "ProjectDirs not found"))?;
    Ok(proj_dirs.config_dir().to_path_buf())
}

// Helper: Ensure config.toml exists
fn ensure_config_file_exists(config_dir: &PathBuf) -> io::Result<PathBuf> {
    let toml_config_path = config_dir.join("config.toml");
    if !toml_config_path.exists() {
        println!("Please run `focus setup --list <exact path to website list>` to setup focus");
        std::process::exit(1);
    }
    Ok(toml_config_path)
}

// Helper: Ensure config directory exists
fn ensure_dir_exists(config_dir: &PathBuf) -> io::Result<()> {
    if !config_dir.exists() {
        fs::create_dir_all(config_dir).expect("Error while creating config directory");
    }
    Ok(())
}

/// Returns the path to the config.toml file, ensuring it exists and instructing the user if missing.
fn get_toml_config_path() -> io::Result<PathBuf> {
    let config_dir = find_config_dir()?;
    ensure_dir_exists(&config_dir)?;
    ensure_config_file_exists(&config_dir)
}

/// Returns the path to the hosts backup file, ensuring the config directory exists.
fn prepare_hosts_backups() -> io::Result<PathBuf> {
    let config_dir = find_config_dir()?;
    ensure_dir_exists(&config_dir)?;
    Ok(config_dir.join("hosts_backup"))
}

// Helper: Update hosts file with blocked websites
fn rewrite_hosts_contents_to_block_websites(
    hosts_content: &str,
    websites_list_content: &str,
) -> String {
    let mut hosts_file_with_blocked_websites = hosts_content.to_owned();
    hosts_file_with_blocked_websites.push_str("\n# ========== Temp Hosts =========");
    for website in websites_list_content.lines() {
        let website = website.trim();
        if !website.is_empty() && !hosts_content.contains(website) {
            hosts_file_with_blocked_websites.push_str(&format!("\n127.0.0.1\t{}", website));
        }
    }
    hosts_file_with_blocked_websites.push_str("\n# ========== Temp Hosts =========");
    hosts_file_with_blocked_websites
}

// Helper: Restore hosts file from backup
fn restore_hosts_file(backup_path: &PathBuf, hosts_path: &str) -> io::Result<()> {
    let backup_file_content = fs::read_to_string(backup_path)?;
    let mut backup_file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(hosts_path)?;
    backup_file.write_all(backup_file_content.as_bytes())?;
    Ok(())
}

/// Reads config.toml and returns the websites file path, or an error if not set.
fn get_websites_file_path_from_config(config_path: PathBuf) -> io::Result<String> {
    let config_content = fs::read_to_string(config_path)?;
    let config_toml_data: Value = toml::from_str(&config_content)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid TOML in config file"))?;
    match config_toml_data
        .get("website_list_path")
        .and_then(Value::as_str)
    {
        Some(path) => Ok(path.to_string()),
        None => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "website_list_path not set in config",
        )),
    }
}
