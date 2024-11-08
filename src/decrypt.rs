use core::str;

use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use std::fs;

pub fn decrypt_password(priv_key_path: &str, encrypted_password_path: &str) -> String {
    let private_key_text = fs::read(priv_key_path).expect("failed to read private key file");
    let private_key_pem = str::from_utf8(&private_key_text).expect("invalid UTF-8");
    let private_key =
        RsaPrivateKey::from_pkcs1_pem(private_key_pem).expect("failed to parse private key");

    let encrypted_password =
        fs::read(encrypted_password_path).expect("failed to read encrypted password file");
    let password_text = private_key
        .decrypt(Pkcs1v15Encrypt, &encrypted_password)
        .expect("failed to decrypt password");
    let password = str::from_utf8(&password_text).expect("invalid UTF-8");

    password.to_string()
}
