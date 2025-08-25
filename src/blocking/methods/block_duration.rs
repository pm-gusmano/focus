use std::time::Duration;

pub fn parse_time_string(time: &String) -> Duration {
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
    Duration::from_millis(time_in_milliseconds)
}

// struct HourMinSec {
//     hours: u64,
//     minutes: u64,
//     seconds: u64,
// }


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_string_to_hour_min_sec_struct() {
    //     let test_hour_min_sec_string = "1h30m45s".to_string();
    //     let test_hour_min_sec_struct = test_hour_min_sec_string.to
    // }

    #[test]
    fn test_parse_minutes() {
        let d = parse_time_string(&"5m".to_string());
        assert_eq!(d, Duration::from_millis(5 * 60 * 1000));
    }

    #[test]
    fn test_parse_seconds() {
        let d = parse_time_string(&"10s".to_string());
        assert_eq!(d, Duration::from_millis(10 * 1000));
    }

    #[test]
    fn test_parse_hours() {
        let d = parse_time_string(&"2h".to_string());
        assert_eq!(d, Duration::from_millis(2 * 60 * 60 * 1000));
    }

    #[test]
    fn test_parse_invalid() {
        let d = parse_time_string(&"abc".to_string());
        assert_eq!(d, Duration::from_millis(0));
    }
}
