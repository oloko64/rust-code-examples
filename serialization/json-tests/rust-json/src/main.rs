mod json_rpc;
mod try_from;

use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

use crate::json_rpc::JsonRpcRequest;

use serde_json;

// Test the serialization and deserialization using serde_json.
fn main() {
    let file = read_to_string("../test.json").expect("Error reading file");
    let teste: Teste = serde_json::from_str(&file).expect("Error converting file");
    println!("{:#?}", teste);

    let named =
        r#"{"jsonrpc": "2.0", "method": "hello", "params": {"hello": "test", "value": 42 }}"#;
    let positional = r#"{"jsonrpc": "2.0", "method": "hello_2", "params": ["test", 42]}"#;

    dbg!(serde_json::from_str::<JsonRpcRequest>(named).unwrap());
    dbg!(serde_json::from_str::<JsonRpcRequest>(positional).unwrap());
}

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
