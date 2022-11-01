use std::env;
use std::fs;

/// Open a file for reading and return its contents as a string.
fn read_file(path: &str) -> String {
    let file = fs::read_to_string(path).expect("Unable to read input file");
    file
}

/// Convert a string of bytes to a string of binary.
fn convert_string_to_binary(text: String) -> String {
    let mut binary = String::new();
    for character in text.clone().into_bytes() {
        binary += &format!("0{:b} ", character);
    }
    binary.trim().to_string()
}

/// Write the contents of a string to a file.
fn write_file(path: &str, text: String) {
    fs::write(path, text).expect("Unable to write output file");
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        println!("Usage: {} <input-file>", args[0]);
        return;
    }

    let output = args[1].split(".").collect::<Vec<_>>()[0].to_string() + "-binary.txt";
    write_file(&output, convert_string_to_binary(read_file(&args[1])));

    println!("Done!, output file: {}", output);
}
