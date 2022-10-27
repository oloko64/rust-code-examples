use std::fs::read_to_string;

use serde::{Serialize, Deserialize};


extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Teste {
    #[serde(rename = "products")]
    products: Vec<Product>,
}

// Adding those serde macros above avery variable inside a struct improve it's performance.
// You can use the "Paste JSON as Code" extension to do this automatically.
#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "price")]
    price: String,

    #[serde(rename = "description")]
    description: String,
}

// Test the serialization and deserialization using serde_json.
fn main() {
    let file = read_to_string("../test.json").expect("Error reading file");
    let teste: Teste = serde_json::from_str(&file).expect("Error converting file");
    println!("{:#?}", teste);
}
