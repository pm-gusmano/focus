use clap::{Args, Parser, Subcommand};
use directories::ProjectDirs;

use serde::Serialize;
use std::path::PathBuf;

use std::fs::{self, OpenOptions};
use toml;

use std::io::Write;

use focus::{blocking::manage_websites, os_backend};

// Top level struct to hold the TOML data.
#[derive(Serialize)]
struct Config {
    website_list_path: String,
}

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "focus - a simple CLI to stay focused and productive",
    long_about = "focus - a simple CLI to stay focused and productive"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    /// time for which you want to block websites
    #[arg(long = "time")]
    time: Option<String>,
    /// task name for which you want to block websites
    #[arg(long = "task")]
    task: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Setup focus
    Setup(Setup),
    /// Reset OS hosts file to original
    Reset,
}

#[derive(Args)]
struct Setup {
    /// task name
    #[arg(long = "list")]
    list: String,
}

// fn is_file_exist(path: &str) -> bool {
//     return std::path::Path::new(path).is_file();
// }

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Setup(setup)) => {
            // println!("List path: {}", setup.list);
            if let Some(proj_dirs) = ProjectDirs::from("com", "chetanxpro", "focusguard") {
                let config_dir = proj_dirs.config_dir();

                if !config_dir.exists() {
                    fs::create_dir_all(config_dir).expect("Error while creating config directory");

                    fs::File::create(config_dir.join("hosts_backup"))
                        .expect("Error while creating hosts backup file");
                }

                let config = Config {
                    website_list_path: setup.list.clone(),
                };

                let toml = toml::to_string(&config).unwrap();

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
        Some(Commands::Reset) => {
            let hosts_path: &str = os_backend::get_hosts_path();
            let mut backup_path: PathBuf = PathBuf::new();

            if let Some(proj_dirs) = ProjectDirs::from("com", "chetanxpro", "focusguard") {
                let config_dir = proj_dirs.config_dir();

                backup_path = config_dir.join("hosts_backup");
            }

            let backup_file_content: String =
                fs::read_to_string(backup_path).expect("Error while reading backup file");

            let mut host_file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(hosts_path)
                .unwrap();

            host_file.write_all(backup_file_content.as_bytes()).unwrap();

            println!("Hosts file reset ✅")
        }
        None => {
            if let (Some(time), Some(task)) = (cli.time, cli.task) {
                // println!("lol")
                // println!("Time: {:#?} , Task: {}", time, task);

                let mut time_in_milliseconds: u64;
                // let time: String = cli.time;
                // let task: String = args.task;

                if time.contains("m") {
                    time_in_milliseconds = time.replace("m", "").parse().unwrap();
                    time_in_milliseconds = time_in_milliseconds * 60 * 1000
                } else if time.contains("s") {
                    time_in_milliseconds = time.replace("s", "").parse().unwrap();
                    time_in_milliseconds = time_in_milliseconds * 1000
                } else if time.contains("h") {
                    time_in_milliseconds = time.replace("h", "").parse().unwrap();
                    time_in_milliseconds = time_in_milliseconds * 60 * 60 * 1000
                } else {
                    time_in_milliseconds = 0
                }

                manage_websites::block_websites(time_in_milliseconds, &task, &time).expect("Error")
            } else {
                println!("No command provided");
            }
        }
    }
}
