use serde::{Deserialize, Serialize};

// This is a struct that will be used to test the serialization and deserialization.
#[derive(Serialize, Deserialize, Debug)]
struct Screen {
    title: String,
    width: u32,
    height: u32,
    model: String,
}

/// Serialize the struct into a JSON string.
pub fn to_json() {
    let screen = Screen {
        title: "My Screen".to_string(),
        width: 1920,
        height: 1080,
        model: "Samsung".to_string(),
    };
    // This makes sure that the serialization of the struct is correct, otherwise it will panic.
    let json = serde_json::to_string_pretty(&screen).unwrap();
    let serialized: Screen = serde_json::from_str(&json).unwrap();
    println!("{}", json);
    println!("{:?}", serialized);
}
