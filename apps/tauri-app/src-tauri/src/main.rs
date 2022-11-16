#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_file(content: Vec<u8>, file_name: &str) -> bool {
    match write_file(&content, file_name) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[tauri::command]
fn read_file(path: &str) -> Vec<u8> {
    let mut contents = File::open(path).unwrap();
    let mut buffer = Vec::new();
    contents.read_to_end(&mut buffer).unwrap();
    buffer
}

fn write_file(content: &[u8], file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_name)?;
    file.write_all(content)?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, save_file, read_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
