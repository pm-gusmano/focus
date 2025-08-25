use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use serde::Serialize;

use clap::Args;
use directories::ProjectDirs;

pub fn cmd_setup(setup_args: &SetupConfigArgs) {
    if let Some(proj_dirs) = ProjectDirs::from("com", "chetanxpro", "focusguard") {
        let config_dir = proj_dirs.config_dir();

        if !config_dir.exists() {
            fs::create_dir_all(config_dir).expect("Error while creating config directory");

            fs::File::create(config_dir.join("hosts_backup"))
                .expect("Error while creating hosts backup file");
        }

        let block_config = BlockConfig {
            website_list_path: setup_args.list.clone(),
        };

        let toml = toml::to_string(&block_config).unwrap();

        fs::File::create(config_dir.join("config.toml")).unwrap();

        let mut config_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(config_dir.join("config.toml"))
            .unwrap();

        config_file.write_all(toml.as_bytes()).unwrap();

        println!("Website file path saved in config ✅")
    }
}

#[derive(Args)]
pub struct SetupConfigArgs {
    /// task name
    #[arg(long = "list")]
    pub list: String,
}

// Top level struct to hold the TOML data.
#[derive(Serialize)]
pub struct BlockConfig {
    pub website_list_path: String,
}
