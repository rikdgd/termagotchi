use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

/// Logs a message to the `logs.txt` file, and adds a timestamp to it.
/// ## parameters:
/// * `message` - The message that should be logged.
#[allow(unused)]
pub fn log_to_file(message: &str) -> std::io::Result<()> {
    let mut log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open("./logs.txt")?;

    let timestamp = Utc::now().to_string();
    let log_message = format!("{} - {}\n", timestamp, message);
    
    log_file.write_all(log_message.as_bytes())?;
    log_file.flush()?;
    
    Ok(())
}