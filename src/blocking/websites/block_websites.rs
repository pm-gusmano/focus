use std::{
    fs,
    io::{self, Write},
};

use crate::{blocking::methods::block_duration, os_backend};

use super::hosts_specific_implementation::hosts::{
    restore_hosts_file, rewrite_hosts_contents_to_block_websites,
};
use crate::blocking::{config::config, ui::spinners::show_interruptible_spinner_for_duration};

// Make a struct in the blocking module that describes exactly what is to be
// blocked (websites first, then apps later; how long, and a short-hand name
// for that combination of what's to be blocked and for how long. The programs
// that are blocked, the conditions for their blocking (at this point, just a
// duration of time, but later more, like a schedule or location,) and the name
// for that combination can be stored in a yaml or toml file in a few config
// directories, and represented by a struct in Rust.


pub trait Blockable {
    fn block(&self) -> std::io::Result<()>;
    fn unblock(&self) -> std::io::Result<()>;
}

struct Websites {
    hosts_path: String,
    backup_path: String,
    pub blocked_websites_list: String,
}

impl Websites {
    pub fn new() -> Self {
        let hosts_path = os_backend::get_hosts_path().to_string();
        let backup_path = Websites::backup_hosts(hosts_path.clone())
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let blocked_websites_list =
            Websites::get_blocked_website_list_from_toml_config().unwrap_or_default();

        Websites {
            hosts_path,
            backup_path,
            blocked_websites_list,
        }
    }

    pub(super) fn backup_hosts(hosts_path: String) -> io::Result<std::path::PathBuf> {
        let backup_path = config::find_config_dir()?.join("hosts_backup");
        fs::create_dir_all(backup_path.parent().unwrap())?;
        fs::copy(hosts_path, &backup_path)?;
        Ok(backup_path)
    }

    fn get_blocked_website_list_from_toml_config() -> Result<String, io::Error> {
        // Get the path to the config file
        let toml_config_path = config::get_toml_config_path()?;

        // Read the config file to generate the file path to the websites list
        let blocked_websites_file_path = config::get_string_from_config(toml_config_path)?;

        // Get the list of blocked websites
        let blocked_websites_list = fs::read_to_string(blocked_websites_file_path)?;
        Ok(blocked_websites_list)
    }
}

impl Blockable for Websites {
    fn block(&self) -> std::io::Result<()> {
        let hosts_content = fs::read_to_string(&self.hosts_path)?;
        let hosts_file_with_blocked_websites =
            rewrite_hosts_contents_to_block_websites(&hosts_content, &self.blocked_websites_list);
        println!("Content:\n {}", hosts_file_with_blocked_websites);
        fs::write(&self.hosts_path, &hosts_file_with_blocked_websites)?;
        Ok(())
    }

    fn unblock(&self) -> std::io::Result<()> {
        let backup_file_content = fs::read_to_string(&self.backup_path)?;
        let mut backup_file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.hosts_path)?;
        backup_file.write_all(backup_file_content.as_bytes())?;
        Ok(())
    }
}

fn show_blocking_spinner(user_input_time: &String, task: Option<&String>) -> io::Result<()> {
    let formatted_message = generate_blocking_message(user_input_time, task);
    let duration_to_wait = block_duration::parse_time_string(user_input_time);
    show_interruptible_spinner_for_duration(&duration_to_wait, &formatted_message)?;
    Ok(())
}

fn generate_blocking_message(user_input_time: &String, task: Option<&String>) -> String {
    // Make a message to inform the user what's being blocked.
    let formatted_message = match task {
        Some(t) => format!("Blocked websites for {} for task: {}", user_input_time, t),
        None => format!("Blocked websites for {}", user_input_time),
    };
    formatted_message
}
