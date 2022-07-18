use std::collections::HashMap;
use std::collections::HashSet;

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
    println!("{}", book_reviews.get("Adventures of Huckleberry Finns").unwrap_or(&"Nothing found".to_string()));
}
