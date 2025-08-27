use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use serde::Serialize;
use clap::Args;
use crate::utils::config_file_helper;

pub fn cmd_setup(setup_args: &SetupConfigArgs) -> Result<(), String> {
    // Use helper to get config directory and ensure it exists
    let config_dir: PathBuf = config_file_helper::find_config_dir().expect("Could not find config directory");
    fs::create_dir_all(&config_dir).expect("Could not create config directory");

    // Create hosts_backup file if it doesn't exist
    let hosts_backup_path = config_dir.join("hosts_backup");
    if !hosts_backup_path.exists() {
        fs::File::create(&hosts_backup_path).expect("Error while creating hosts backup file");
    }

    // Prepare config TOML
    let block_config = BlockConfig {
        website_list_path: setup_args.list.clone(),
    };
    let toml = toml::to_string(&block_config).unwrap();

    // Write config TOML
    let config_path = config_dir.join("config.toml");
    let mut config_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&config_path)
        .expect("Error while creating/opening config.toml");
    config_file.write_all(toml.as_bytes()).unwrap();

    println!("Website file path saved in config ✅");
    Ok(())
}

#[derive(Args)]
pub struct SetupConfigArgs {
    /// A txt file containing a list of websites to block, one per line
     /// e.g. /home/user/.config/focus/websites.txt
    #[arg(long = "list")]
    pub list: String,
}

// Top level struct to hold the TOML data.
#[derive(Serialize)]
pub struct BlockConfig {
    pub website_list_path: String,
}
