use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use crate::blocking::config::config;

/// Returns the path to the hosts backup file, ensuring the config directory exists.
pub(in crate::blocking::websites) fn prepare_hosts_backups() -> io::Result<PathBuf> {
    let config_dir = config::find_config_dir()?;
    config::ensure_dir_exists(&config_dir)?;
    Ok(config_dir.join("hosts_backup"))
}

// Helper: Update hosts file with blocked websites
pub(in crate::blocking::websites) fn rewrite_hosts_contents_to_block_websites(
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
pub(in crate::blocking::websites) fn restore_hosts_file(backup_path: &PathBuf, hosts_path: &str) -> io::Result<()> {
    let backup_file_content = fs::read_to_string(backup_path)?;
    let mut backup_file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(hosts_path)?;
    backup_file.write_all(backup_file_content.as_bytes())?;
    Ok(())
}