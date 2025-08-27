use duration_string::DurationString;

use crate::blocking::websites::{self, Blockable};
use crate::ui::spinners::show_interruptible_spinner_for_duration;
use crate::ui::blocking_message::generate_blocking_message;

pub fn cmd_start(start_args: &StartBlockArgs) -> Result<(), String> {
    let website_logic = websites::Websites::new();

    let duration_to_wait = DurationString::from_string(start_args.time_string.to_string())
        .map_err(|e| format!("Failed to parse duration: {}", e))?
        .into();

    let blocking_message =
        generate_blocking_message(&start_args.time_string, start_args.task.as_ref());

    website_logic
        .block()
        .map_err(|e| format!("Website blocking failed: {}", e))?;

    show_interruptible_spinner_for_duration(&duration_to_wait, &blocking_message)
        .map_err(|e| format!("Spinner Error: {}", e))?;

    website_logic
        .unblock()
        .map_err(|e| format!("Website unblocking failed: {}", e))?;

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
