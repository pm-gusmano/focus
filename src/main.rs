use focus::cli::{cli_runner, cli_utils::ToExitCode};

fn main() -> std::process::ExitCode {
    cli_runner::run_cli().to_exit_code()
}

