use std::fs::OpenOptions;
use std::io::Write;
use std::process::{Command, Stdio};

use chrono::Local;

pub fn connect(
    password: String,
    connection_name: &str,
    username: &str,
) -> Result<std::process::Output, Box<dyn std::error::Error>> {
    let output = Command::new("rasdial")
        .arg(connection_name)
        .arg(username)
        .arg(password)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()?;
    Ok(output)
}

pub fn dialing_log(output: std::process::Output) {
    let current_time = Local::now();

    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("LOG_FILE")
        .expect("Failed to open log file");
    if output.status.success() {
        let log_message = format!(
            "{} - Dialing successful\n",
            current_time.format("%Y-%m-%d %H:%M:%S")
        );
        log_file
            .write_all(log_message.as_bytes())
            .expect("Failed to write to log file");

        println!("Dialing successful!");
    } else {
        let error_output = String::from_utf8_lossy(&output.stderr);
        let log_message = format!(
            "{} - Dialing failed: {}\n",
            current_time.format("%Y-%m-%d %H:%M:%S"),
            error_output
        );
        log_file
            .write_all(log_message.as_bytes())
            .expect("Failed to write to log file");

        println!("Dialing failed: {}", error_output);
    }
}
