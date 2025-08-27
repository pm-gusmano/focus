use std::{
    fs,
    io::{self, Write},
};

use crate::os_backend;
use crate::utils::config_file_helper;

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

pub struct Websites {
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

    fn backup_hosts(hosts_path: String) -> io::Result<std::path::PathBuf> {
        let backup_path = config_file_helper::find_config_dir()?.join("hosts_backup");
        fs::create_dir_all(backup_path.parent().unwrap())?;
        fs::copy(hosts_path, &backup_path)?;
        Ok(backup_path)
    }

    fn get_blocked_website_list_from_toml_config() -> Result<String, io::Error> {
        // Get the path to the config file
        let toml_config_path = config_file_helper::get_toml_config_path()?;

        // Read the config file to generate the file path to the websites list
        let blocked_websites_file_path =
            config_file_helper::get_string_from_config(toml_config_path)?;

        // Get the list of blocked websites
        let blocked_websites_list = fs::read_to_string(blocked_websites_file_path)?;
        Ok(blocked_websites_list)
    }
}

impl Blockable for Websites {
    fn block(&self) -> std::io::Result<()> {
        let hosts_content = fs::read_to_string(&self.hosts_path)?;
        let mut hosts_file_with_blocked_websites = hosts_content.clone();
        hosts_file_with_blocked_websites.push_str("\n# ========== Temp Hosts =========");
        for website in self.blocked_websites_list.lines() {
            let website = website.trim();
            if !website.is_empty() && !hosts_content.contains(website) {
                hosts_file_with_blocked_websites.push_str(&format!("\n127.0.0.1\t{}", website));
            }
        }
        hosts_file_with_blocked_websites.push_str("\n# ========== Temp Hosts =========");
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
