// use crate::blocking::methods::block_duration;
use crate::{blocking::websites::block_websites, os_backend};

pub fn cmd_start(start_args: &StartBlockArgs) -> Result<(), String> {
    block_websites::block_websites_via_hosts_config_change(
        &start_args.time_string,
        start_args.task.as_ref(),
    )
    .map_err(|e| e.to_string())?;

    let hosts_path = os_backend::get_hosts_path();
    

    println!("\n  Unblocked websites ✅");

    Ok(())
}

#[derive(clap::Args)]
pub struct StartBlockArgs {
    /// How much time to block for
    /// This should be a number followed by s, m or h (for seconds, minutes or hours)
    /// e.g. 25m for 25 minutes
    #[arg(long = "time", short = 't')]
    pub time_string: String,

    /// What task you are working on
    #[arg(long = "task", short = 'k')]
    pub task: Option<String>,
}
