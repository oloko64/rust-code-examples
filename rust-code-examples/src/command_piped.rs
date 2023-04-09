use std::{
    io::BufRead,
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
}
