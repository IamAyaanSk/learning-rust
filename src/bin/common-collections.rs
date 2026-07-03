// Always allocated on heaps --> grow, shrink size as needed

use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // -------- Vectors ---------- //
    let mut v: Vec<i32> = Vec::new(); // Specify type as we are initiating as empty vector
    v.push(1);

    // Macro
    let mut v2 = vec![1, 2, 3];

    // Referencing
    // Indexes
    let third = &v2[2]; // Borrows, the element exist in the heap only
    let _p = v2[2]; // i32 have Copy trait hence this copies the element
    println!("{}", third);

    // Breaks at run time if out of bound index is accessed
    // v2[9];
    // Hemce we use get method, No runtime error
    match v2.get(9) {
        Some(val) => println!("Value is {}", val),
        None => println!("Out of bound index passed!!"),
    }

    // Iterating
    for i in &mut v2 {
        *i += 10;
    }

    for i in &v2 {
        println!("{i}")
    }

    // Storing enum variants in vector
    enum SpreadsheetCell {
        Int(i32),
        Text(String),
        Float(f64),
    }

    let my_vector = vec![
        SpreadsheetCell::Int(23),
        SpreadsheetCell::Int(30),
        SpreadsheetCell::Text(String::from("Ayaan")),
        SpreadsheetCell::Float(2.30),
    ];

    for i in &my_vector {
        match i {
            SpreadsheetCell::Float(val) => println!("Data is of float type: {val}",),
            SpreadsheetCell::Int(val) => println!("Data is of integer type: {val}",),
            SpreadsheetCell::Text(val) => println!("Data is of string type: {val}",),
        }
    }

    // ------------- Strings ------------------- //
    // Collection of bytes UTF-8 encoded
    // ASCII --> Each character is of a byte and 7 bits are used to represent it, 128 unique characters [Only english alpahbets, some special characters and commands]
    // Different languages came with their own encoding
    // Unified under unicode
    // UTF-8 is a variable width encoding used to save unicode characters --> 1-4 byte

    let mut s1 = String::from("Hello, ");
    s1.push_str("World");
    s1.push('!');
    let s2 = "Hello Rust!";
    let s3 = s1 + &s2;
    println!("{s3}");

    // We can also use format macro to concat string
    // let s4 = format!("{} {}", s1, s2); // Can't use s1 as it was moved previously to s3
    // Also format macro doen't moves ownership

    // Indexing string
    // Direct indexing doesn't work since a character can be of variable bytes

    let hello = "Здравствуйте";
    println!("Length of hello in Russian: {}", hello.len()); // 24
    // It has 24 bytes, but actual characters are 12, 2 bytes each
    // We can use .bytes() to get all bytes in string
    let bytes_in_hello = hello.bytes();
    for byte in bytes_in_hello {
        println!("Byte: {}", byte);
    }

    // We can also use chars to get actual characters
    let chars_in_hello = hello.chars();
    for char in chars_in_hello {
        println!("Char: {}", char);
    }

    // Also in languages like hindi, we can get actual modified letter individually (like s in namaste) using a cargo crate
    // To get complete Grapheme clusters
    let graphemes_in_hello = hello.graphemes(true);
    for grapheme in graphemes_in_hello {
        println!("Grapheme: {}", grapheme);
    }

    // ----------- HashMaps ------------ //
    let mut scores = HashMap::new();

    scores.insert(String::from("A"), 23); // If passing from a variable its ownership will be moved to hashmap
    scores.insert(String::from("B"), 30);

    let team_name = String::from("A");
    let _score = scores.get(&team_name);

    for (key, val) in &scores {
        println!("{key} : {val}")
    }

    // entry() returns Entry<> which exposes useful methods
    let b = scores.entry(String::from("C")).or_insert(40); // or_insert() adds a value to the entry if it is absent, it also returns a muteable reference of the value
    *b = 42;
    println!("{}", scores.get("C").unwrap());

    // We need to count number of appearance of words in a sentence
    let sentence = "Hello hello welcome to our world the rust world";
    let lowercase_sentence = sentence.to_lowercase();
    let mut counts: HashMap<&str, i32> = HashMap::new();

    for word in lowercase_sentence.split_whitespace() {
        let current_count_of_word = counts.entry(word).or_insert(0);
        *current_count_of_word += 1;
    }

    println!("{:?}", counts);
}
