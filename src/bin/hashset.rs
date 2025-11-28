#![allow(unused)] // make to ignore some warnings

// HashSets are collections of unique values | generic too
// import hashset
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // Hashset
    // HashSet stores unique values with no defined order | generic too
    let mut set: HashSet<u32> = HashSet::new();

    // Insert values, returns true if value was not present
    let inserted = set.insert(1);
    println!("Inserted 1: {}", inserted); // true

    let inserted = set.insert(1);
    println!("Inserted 1: {}", inserted); // false

    // Check for presence, returns true if value is present
    let contains = set.contains(&1); // pass reference to the type
    println!("Set contains 1: {}", contains); // true

    let contains = set.contains(&2); // pass reference to the type
    println!("Set contains 2: {}", contains); // false

    println!("Set: {:?}", set);
}
