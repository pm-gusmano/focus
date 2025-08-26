use crossterm::{
    event::{poll, read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use spinners::{Spinner, Spinners};

use std::io;
use std::time::{self, Duration};

// Helper: Spinner and timer
/// Shows a spinner with a message, and handles interruption directly.
pub fn show_interruptible_spinner_for_duration(
    duration_to_wait: &Duration,
    message: &String,
) -> io::Result<()> {
    let mut sp = Spinner::new(Spinners::Dots9, message.clone().into());
    let outcome = wait_until_timer_or_interrupt(*duration_to_wait)?;
    sp.stop();

    match outcome {
        TimerOutcome::Completed => Ok(()),
        TimerOutcome::ExitedEarly => {
            println!("\n  Timer exited early by user.");
            Ok(())
        }
    }
}

enum TimerOutcome {
    Completed,
    ExitedEarly,
}

fn wait_until_timer_or_interrupt(timer_duration: Duration) -> io::Result<TimerOutcome> {
    let start_time = time::Instant::now();
    println!("  Press ESC or 'e' to exit early!");
    enable_raw_mode()?;
    while start_time.elapsed() < timer_duration {
        if poll(Duration::from_millis(1_000))? {
            let event = read()?;

            // Check for 'e' or Esc key to exit early
            if let Event::Key(key) = event {
                if key.code == KeyCode::Char('e') || key.code == KeyCode::Esc {
                    disable_raw_mode()?;
                    return Ok(TimerOutcome::ExitedEarly);
                }
            }
        }
    }
    disable_raw_mode()?;
    Ok(TimerOutcome::Completed)
}
