#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use base64::decode;
use std::{
    error::Error,
    fs::File,
    io::{self, Write},
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_file(content: &[u8], file_name: &str) -> bool {
    match write_file(content, file_name) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn write_file(content: &[u8], file_name: &str) -> Result<(), Box<dyn Error>> {
    let Some(binding) = dirs::download_dir() else {
        return Err("No download directory found".into());
    };
    let Some(dl_dir) = binding.to_str() else {
        return Err("No download directory found".into());
    };
    let mut file = File::create(format!("{}/{}", dl_dir, file_name))?;
    file.write_all(&decode(content)?[..])?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, save_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
