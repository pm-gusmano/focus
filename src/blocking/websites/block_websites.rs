use std::{fs, io};

use crate::{blocking::methods::block_duration, os_backend};

use super::hosts_specific_implementation::hosts::{
    prepare_hosts_backups, restore_hosts_file, rewrite_hosts_contents_to_block_websites,
};
use crate::blocking::{config::config, ui::spinners::show_interruptible_spinner_for_duration};

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
    let toml_config_path = config::get_toml_config_path()?;

    // Read the config file to generate the file path to the websites list
    let blocked_websites_file_path = config::get_string_from_config(toml_config_path)?;

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
