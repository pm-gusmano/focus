use std::process::ExitCode;

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