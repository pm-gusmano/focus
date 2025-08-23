use crate::blocking::manage_websites;
use serde::Serialize;

// Circular dependency issues introduced by this import--fix immediately after current refactor
use crate::cli::cli_runner::Cli;

pub fn set_block_for_time_and_task(cli: Cli) {
    if let (Some(time), Some(task)) = (cli.time, cli.task) {
        

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



// Top level struct to hold the TOML data.
#[derive(Serialize)]
pub struct Config {
    pub website_list_path: String,
}
