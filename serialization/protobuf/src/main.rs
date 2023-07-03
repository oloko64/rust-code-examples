pub mod persons {
    include!(concat!(env!("OUT_DIR"), "/persons.rs"));
}
use persons::{Person, Persons};
use prost::Message;

fn main() {
    let persons = Persons {
        persons: vec![
            Person {
                name: "Alice".to_string(),
                age: 42,
            },
            Person {
                name: "Bob".to_string(),
                age: 43,
            },
        ],
    };

    let mut buf = Vec::new();
    persons.encode(&mut buf).unwrap();

    println!("{:?}", buf);

    let decoded_persons = Persons::decode(&buf[..]).unwrap();
    println!("{:?}", decoded_persons);
}

// This is the same but using the #[derive(prost::Message)] macro

// #[derive(prost::Message)]
// struct Person {
//     #[prost(string, tag = "1")]
//     name: String,

//     #[prost(uint32, tag = "2")]
//     age: u32,
// }

// #[derive(prost::Message)]
// struct Persons {
//     #[prost(message, repeated, tag = "1")]
//     persons: Vec<Person>,
// }
