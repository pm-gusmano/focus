use clap::{Parser, Subcommand};

use super::commands;
use super::commands::setup::SetupConfigArgs;
use super::set_block_for_time_and_task::set_block_for_time_and_task;

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "focus - a simple CLI to stay focused and productive",
    long_about = "focus - a simple CLI to stay focused and productive"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// time for which you want to block websites
    #[arg(long = "time")]
    pub time: Option<String>,

    /// task name for which you want to block websites
    #[arg(long = "task")]
    pub task: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Setup focus
    Setup(SetupConfigArgs),

    /// Reset OS hosts file to original
    Reset,
}

pub fn run_cli() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Setup(setup_list)) => commands::setup::cmd_setup(setup_list),
        Some(Commands::Reset) => commands::reset::cmd_reset(),
        None => set_block_for_time_and_task(cli),
    }
}
