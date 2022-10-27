use std::{fs::File, io::{Write, Read}, error::Error};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct World {
    name: String,
    age: u8,
}

fn main() {
    using_bincode_manual_to_file().unwrap();
    using_bincode_to_file().unwrap();
}

/// Manually opening the file and deserializing it without using the bincode functions.
fn using_bincode_manual_to_file() -> Result<(), Box<dyn Error>> {
    let world = World { name: "manual".to_string(), age: 29 };

    let mut file = File::create("manual.bin")?;
    let serialized = bincode::serialize(&world)?;
    file.write_all(&serialized)?;

    let mut read_file = File::open("manual.bin")?;
    let mut buffer = Vec::new();
    read_file.read_to_end(&mut buffer)?;
    let decoded: World = bincode::deserialize(&buffer)?;
    println!("{:?}", decoded);
    Ok(())
}

/// Using the functions from bincode to serialize and deserialize.
fn using_bincode_to_file() -> Result<(), Box< dyn Error>>{
    let world = World { name: "bincode".to_string(), age: 23 };

    let file = File::create("auto.bin")?;
    bincode::serialize_into(file, &world)?;

    let read_file = File::open("auto.bin")?;
    let decoded: World = bincode::deserialize_from(read_file)?;
    println!("{:?}", decoded);
    Ok(())
}