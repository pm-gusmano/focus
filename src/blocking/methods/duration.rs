pub fn parse_time_string(time: &String) -> u64 {
    let mut time_in_milliseconds: u64;

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
    time_in_milliseconds
}