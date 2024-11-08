mod bar;
mod config;
mod decrypt;
mod dialing;

use std::io::{self, Read, Write};

use config::Settings;
use indicatif::ProgressStyle;
use reqwest::blocking::Client;

fn main() {
    let setting = Settings::new();

    let password =
        decrypt::decrypt_password(&setting.private_key_path, &setting.encrypted_password_path);

    let pb = bar::create_bar();
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {percent}% ({eta})",
            )
            .expect("Failed to create progress bar template")
            .progress_chars("##-"),
    );

    pb.set_position(0);

    let log = match dialing::create_log(){
        Ok(log) => log,
        Err(_) => std::process::exit(1),
    };

    let output = match dialing::connect(password, &setting.connection_name, &setting.username) {
        Ok(output) => output,
        Err(_) => std::process::exit(1),
    };

    let client = Client::new();
    let response = client
        .get(&setting.test_url)
        .send()
        .expect("Failed to send request");

    if response.status().is_success() {
        pb.set_position(100);
        pb.finish_with_message("Request successful!");
    } else {
        pb.set_position(100);
        pb.finish_with_message("Request failed!");
    }

    dialing::log_result(log, output);

    println!("Press any key to continue...");
    let _ = io::stdout().flush();
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}
