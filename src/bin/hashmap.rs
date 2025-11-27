#![allow(unused)] // make to ignore some warnings
// import hashmap
use std::collections::HashMap;

// HashMaps are key-value stores | generic too

fn main() {
    // initialize
    let mut scores: HashMap<String, u32> = HashMap::new();

    // Insert key-value pairs
    scores.insert(String::from("red"), 100);
    scores.insert(String::from("green"), 30);
    println!("{:?}", scores);

    // Get - Accessing values
    let value = scores.get("green");
    println!("value is {:?}", value);

    let value = scores.get("yellow");
    println!("value is {:?}", value); // None

    // Update - modify existing value
    scores.insert(String::from("green"), 50); // overwrite
    let value = scores.get("green");
    println!("value is {:?}", value);

    // Upsert - insert if not present or update if present
    let v = scores.entry(String::from("blue")).or_insert(25); // insert
    *v += 200;
    println!("{:?}", v); // update
    println!("{:?}", scores); // blue: 25
}
