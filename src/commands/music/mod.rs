pub mod play;
pub mod force_skip;
pub mod reorder;
pub mod queue;
pub mod remove;

fn millis_to_string(millis: u64) -> String {
    let seconds = (millis / 1000) % 60;
    let minutes = (millis / (1000 * 60)) % 60;
    let hours = (millis / (1000 * 60 * 60)) % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{:02}:{:02}", minutes, seconds)
    }
}