use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use directories;

use spinners::{Spinner, Spinners};

use std::{
    fs::{self, OpenOptions},
    io::{self, Result, Write},
    path::PathBuf,
    time::{self, Duration},
};

use toml::Value;

use crate::{blocking::methods::duration, os_backend};

// https://github.com/crossterm-rs/crossterm/blob/0.19/examples/event-poll-read.rs#L26
pub fn print_events_with_timer(timer_duration: Duration) -> Result<()> {
    let start_time = time::Instant::now();
    println!("  ESC or 'e' to exit early");
    loop {
        // Wait up to 1s for another event
        if poll(Duration::from_millis(1_000))? {
            // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
            let event = read()?;

            if event == Event::Key(KeyCode::Char('e').into()) {
                break;
            }
            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        } else {
            if start_time.elapsed() >= timer_duration {
                break;
            } else {
            }
        }
    }

    Ok(())
}

// Make a struct in the blocking module that describes exactly what is to be
// blocked (websites first, then apps later; how long, and a short-hand name
// for that combination of what's to be blocked and for how long. The programs
// that are blocked, the conditions for their blocking (at this point, just a
// duration of time, but later more, like a schedule or location,) and the name
// for that combination can be stored in a yaml or toml file in a few config
// directories, and represented by a struct in Rust.

pub fn block_websites_via_host_config_change(
    user_input_time: &String,
    task: Option<&String>,
) -> io::Result<()> {
    let hosts_path = os_backend::get_hosts_path();
    let (backup_path, toml_config_path) = get_config_paths(hosts_path)?;

    let websites_path_opt = get_websites_path(toml_config_path);
    let websites_file_path = match &websites_path_opt {
        Some(path) => path.as_str(),
        None => "default",
    };

    let hosts_content = fs::read_to_string(hosts_path)?;
    let websites_list_content = fs::read_to_string(websites_file_path)?;

    make_backup_hosts_file(&backup_path, &hosts_content)?;

    let updated_hosts_content = update_hosts_content(&hosts_content, &websites_list_content);

    write_to_file(hosts_path, &updated_hosts_content)?;

    show_blocking_spinner(&user_input_time, task)?;

    restore_hosts_file(&backup_path, hosts_path)?;

    println!("\n  Unblocked websites ✅");
    Ok(())
}

// Helper: Setup config paths
fn get_config_paths(hosts_path: &str) -> io::Result<(PathBuf, PathBuf)> {
    if let Some(proj_dirs) = directories::ProjectDirs::from("com", "chetanxpro", "focusguard") {
        let config_dir = proj_dirs.config_dir();

        if !config_dir.join("config.toml").exists() {
            println!("Please run `focus setup --list <exact path to website list>` to setup focus");
            std::process::exit(1);
        }

        if !config_dir.exists() {
            fs::create_dir_all(config_dir).expect("Error while creating config directory");

            let backup_path = config_dir.join("hosts_backup");
            fs::File::create(&backup_path).expect("Error while creating hosts backup file");

            let mut backup_host_file_for_emergency =
                fs::File::create(config_dir.join("hosts_backup_for_revert"))
                    .expect("Error while creating hosts backup file");

            backup_host_file_for_emergency
                .write_all(fs::read_to_string(hosts_path).unwrap().as_bytes())
                .expect("Error while writing to backup file");
        }

        let backup_path = config_dir.join("hosts_backup");
        let toml_config_path = config_dir.join("config.toml");
        Ok((backup_path, toml_config_path))
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "ProjectDirs not found",
        ))
    }
}

// Helper: Backup hosts file
fn make_backup_hosts_file(backup_path: &PathBuf, hosts_content: &str) -> io::Result<()> {
    let mut backup_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(backup_path)?;
    backup_file.write_all(hosts_content.as_bytes())?;
    Ok(())
}

// Helper: Update hosts file with blocked websites
fn update_hosts_content(hosts_content: &str, websites_list_content: &str) -> String {
    let mut new_hosts_content = hosts_content.to_owned();
    let website_list = websites_list_content.split('\n');
    new_hosts_content.push_str("\n# ========== Temp Hosts =========");
    for website in website_list {
        println!("Website: {}", website);
        if !hosts_content.contains(website) {
            new_hosts_content.push_str(&format!("\n127.0.0.1\t{}", website));
        }
    }
    new_hosts_content.push_str("\n# ========== Temp Hosts =========");
    println!("Content:\n {}", new_hosts_content);
    new_hosts_content
}

// Helper: Write to file
fn write_to_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

// Helper: Spinner and timer
fn show_blocking_spinner(
    user_input_time: &String,
    task: Option<&String>,
) -> io::Result<()> {
    // Make a message to inform the user what's being blocked.
    let mut formatted_message = format!("Blocked websites for {}", user_input_time);
    if let Some(t) = task {
        let task_message = format!(" for task: {}", t);
        formatted_message = format!("{}{}", formatted_message, task_message);
    }

    // Display the message and spinner
    println!("{}", formatted_message);
    let mut sp = Spinner::new(Spinners::Dots9, formatted_message.clone().into());

    let time_to_sleep_in_milliseconds = duration::parse_time_string(&user_input_time);
    wait_with_user_input_and_timer(time_to_sleep_in_milliseconds)?;
    sp.stop();
    Ok(())
}

// Separate function for user input and timer logic
fn wait_with_user_input_and_timer(time_to_sleep: u64) -> io::Result<()> {
    enable_raw_mode()?;
    let timer_duration = Duration::from_millis(time_to_sleep);
    let result = print_events_with_timer(timer_duration);
    disable_raw_mode()?;
    if let Err(e) = &result {
        println!("Error: {:?}\r", e);
    }
    result
}

// Helper: Restore hosts file from backup
fn restore_hosts_file(backup_path: &PathBuf, hosts_path: &str) -> io::Result<()> {
    let backup_file_content = fs::read_to_string(backup_path)?;
    let mut backup_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(hosts_path)?;
    backup_file.write_all(backup_file_content.as_bytes())?;
    Ok(())
}

pub fn get_websites_path(config_path: PathBuf) -> Option<String> {
    let config_content = fs::read_to_string(config_path).unwrap();
    let value: Value = toml::from_str(&config_content).unwrap();

    let website_list_path = value
        .get("website_list_path")
        .and_then(Value::as_str)
        .map(String::from);
    // .unwrap();

    website_list_path
}
