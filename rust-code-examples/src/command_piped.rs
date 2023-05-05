use std::{
    io::{BufRead, Read},
    process::{Command, Stdio},
};

pub fn command_piped() {
    let mut child = Command::new("speedtest-rs")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let stdout = child.stdout.take().unwrap();
    let reader = std::io::BufReader::new(stdout);

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with('T') {
            println!("{line}");
        }
    }

    child.wait().unwrap();

    //------------------------------------------------------------------------------------------------

    let mut child = Command::new("ping")
        .arg("google.com")
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let output = child.stdout.take().unwrap();
    let mut reader = std::io::BufReader::new(output);
    let mut line = String::new();

    while let Ok(bytes) = reader.read_line(&mut line) {
        if bytes == 0 {
            break;
        }
        print!("{}", line);
        line.clear();
    }

    child.wait().expect("Failed to wait");

    //------------------------------------------------------------------------------------------------

    // Not a good way to do it

    let mut child = Command::new("ping")
        .arg("google.com")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let output = child.stdout.take().unwrap();
    let error = child.stderr.take().unwrap();
    let mut reader_output = std::io::BufReader::new(output);
    let mut reader_error = std::io::BufReader::new(error);
    let mut line = String::new();

    loop {
        let bytes = reader_output.read_line(&mut line).unwrap();
        if bytes == 0 {
            break;
        }
        print!("{}", line);
        line.clear();
    }

    loop {
        let bytes = reader_error.read_line(&mut line).unwrap();
        if bytes == 0 {
            break;
        }
        print!("{}", line);
        line.clear();
    }

    child.wait().expect("Failed to wait");

    //------------------------------------------------------------------------------------------------

    // A better way to do it

    // Or to easily create a stdout merged with stderr check subprocess crate
    // https://crates.io/crates/subprocess

    let mut child = Command::new("ping")
        .arg("google.com")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let output = child.stdout.take().unwrap();
    let error = child.stderr.take().unwrap();
    let reader_output = std::io::BufReader::new(output);
    let reader_error = std::io::BufReader::new(error);

    let mut merged_buf = reader_output.chain(reader_error);
    let mut line = String::new();

    // You can use read_until to read until a specific byte is found
    while let Ok(bytes) = merged_buf.read_line(&mut line) {
        if bytes == 0 {
            break;
        }
        print!("{}", line);
        line.clear();
    }

    child.wait().expect("Failed to wait");
}
