use crate::blocking::websites::block_websites;
use serde::Serialize;

// Circular dependency issues introduced by this import--fix immediately after current refactor
use crate::cli::cli_runner::Cli;

pub fn set_block_for_time_and_task(cli: Cli) -> Result<(), String> {
    if let (Some(time), Some(task)) = (cli.time, cli.task) {

        block_websites::block_websites_via_hosts_config_change(
            &time,
            Some(&task),
        )
        .map_err(|e| format!("Error: {}", e))?;
        Ok(())
    } else {
        println!("No command provided");
        Err("No command provided".to_string())
    }
}




// Top level struct to hold the TOML data.
#[derive(Serialize)]
pub struct Config {
    pub website_list_path: String,
}
