use clap::{Parser, Subcommand};

use super::commands;
use std::process::ExitCode;

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
    /// Start a block session
    Start(commands::start::StartBlockArgs),

    /// Setup a block session
    Setup(commands::setup::SetupConfigArgs),

    /// Reset OS hosts file to original
    Reset,
}

pub fn run_cli() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Start(start_args)) => commands::start::cmd_start(start_args),
        Some(Commands::Setup(setup_args)) => commands::setup::cmd_setup(setup_args),
        Some(Commands::Reset) => commands::reset::cmd_reset(),
        None => Err("No commands provided!".to_string()),
    }
}

pub trait ToExitCode {
    fn to_exit_code(self) -> ExitCode;
}

impl ToExitCode for Result<(), String> {
    fn to_exit_code(self) -> ExitCode {
        match self {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("Error: {}", e);
                ExitCode::FAILURE
            }
        }
    }
}
