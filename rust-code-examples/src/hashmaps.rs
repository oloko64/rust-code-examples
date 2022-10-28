use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::fs::read_to_string;
use std::io::Read;
use std::process;


// Reference: https://doc.rust-lang.org/std/collections/struct.HashMap.html
// Reference: https://doc.rust-lang.org/std/collections/struct.HashSet.html
/// Using HashMap and HashSet collections.
pub fn hashmaps() {
    // Type inference lets us omit an explicit type signature (which
    // would be `HashMap<String, String>` in this example).
    let mut book_reviews = HashMap::new();
    let mut hashset = HashSet::new();

    hashset.insert("a");
    hashset.insert("b");
    hashset.insert("c");
    hashset.insert("d");
    hashset.insert("a");

    println!("{:?}", hashset);

    // Review some books.
    // If you repeat a book key, the first review will be overwritten.
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    // Print all reviews.
    println!("{:?}", book_reviews);
    let teste = || -> String {
        read_to_string("src/main.rss").unwrap_or_else(|err| {
            println!("Error: {}", err);
            String::new()
        })
    };
    println!("{}", teste());
    // Prints the review or print that nothing was found for that key.
    println!("{}", book_reviews.get("Adventures of Huckleberry Finns").unwrap_or(&"Nothing found".to_string()));

    if let Err(e) = run("src/main.rs".to_string()) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

/// A simple file reader example that returns a Result.
pub fn run(filename: String) -> Result<(), Box<dyn Error>>{
    let mut f = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);
    Ok(())
}
