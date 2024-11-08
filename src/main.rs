use chrono::Local;
use core::str;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};

const CONNECTION_NAME: &str = "campusnetwork";
const USERNAME: &str = "xxxxxx@cmcc";
const PRIVATE_KEY_TXT: &str = "private.pem";

fn main() {
    let private_key_text = fs::read(PRIVATE_KEY_TXT).expect("failed to read private key file");
    let private_key_pem = str::from_utf8(&private_key_text).expect("invalid UTF-8");
    let private_key =
        RsaPrivateKey::from_pkcs1_pem(private_key_pem).expect("failed to parse private key");

    let encrypted_password =
        fs::read("encrypted.txt").expect("failed to read encrypted password file");
    let password_text = private_key
        .decrypt(Pkcs1v15Encrypt, &encrypted_password)
        .expect("failed to decrypt password");
    let password = str::from_utf8(&password_text).expect("invalid UTF-8");

    let output = Command::new("rasdial")
        .arg(CONNECTION_NAME)
        .arg(USERNAME)
        .arg(password)
        .stdout(Stdio::null()) // 重定向标准输出到 null
        .stderr(Stdio::piped()) // 重定向标准错误到管道
        .creation_flags(0x08000000) // CREATE_NO_WINDOW 标志
        .output()
        .expect("Failed to execute process");

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
    }
}
