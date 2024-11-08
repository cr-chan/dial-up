use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Settings {
    pub connection_name: String,
    pub username: String,
    pub private_key_path: String,
    pub test_url: String,
    pub encrypted_password_path: String,
}

impl Settings {
    pub fn new() -> Self {
        let settings = fs::read_to_string("settings.json").expect("Failed to read settings.json");
        serde_json::from_str(&settings).expect("Failed to parse json file")
    }
}
