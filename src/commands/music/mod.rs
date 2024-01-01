pub mod clear;
pub mod force_skip;
pub mod leave;
pub mod play;
pub mod queue;
pub mod remove;
pub mod reorder;

fn millis_to_string(millis: u64) -> String {
    let seconds = (millis / 1000) % 60;
    let minutes = (millis / (1000 * 60)) % 60;
    let hours = (millis / (1000 * 60 * 60)) % 24;
    let days = (millis / (1000 * 60 * 60 * 24)) % 30;
    let months = (millis / 1000 * 60 * 60 * 24 * 30) % 12;
    let years = millis / 1000 * 60 * 60 * 24 * 365;

    let mut timestamp = format!("{:02}:{:02}", minutes, seconds);

    // hours
    if millis >= 1000 * 60 * 60 {
        timestamp = format!("{:02}:{:02}", hours, timestamp);
    }

    // days
    if millis >= 1000 * 60 * 60 * 24 {
        timestamp = format!("{:02}:{:02}", days, timestamp);
    }

    // months
    if millis >= 1000 * 60 * 60 * 24 * 30 {
        timestamp = format!("{:02}:{:02}", months, timestamp);
    }

    // years
    if millis >= 1000 * 60 * 60 * 24 * 365 {
        timestamp = format!(
            "{}:{:02}",
            years
                .to_string()
                .as_bytes()
                .rchunks(3)
                .rev()
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap()
                .join(","),
            timestamp
        );
    }

    timestamp
}
