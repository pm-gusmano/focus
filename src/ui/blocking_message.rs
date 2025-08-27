pub fn generate_blocking_message(user_input_time: &String, task: Option<&String>) -> String {
    // Make a message to inform the user what's being blocked.
    let formatted_message = match task {
        Some(t) => format!("Blocked websites for {} for task: {}", user_input_time, t),
        None => format!("Blocked websites for {}", user_input_time),
    };
    formatted_message
}
