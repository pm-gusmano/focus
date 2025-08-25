use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};

use directories::ProjectDirs;

use crate::os_backend;

pub fn cmd_reset() {
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
