#![allow(unused)] // make to ignore some warnings

// Lifetimes in Rust
// All references in Rust have a lifetime
// Lifetime tells the rust compiler how long a reference is valid
// Generic lifetimes allow us to define functions and structs that can work with references of different lifetimes.
// 'a is a lifetime parameter : tick a
// It is a convention to use 'a, 'b, 'c for lifetime parameters
// but we can use any valid identifier

fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let x = String::from("🦀");
    {
        let y = String::from("Hello, Rustaceans! 😺😺");
        let z = longest_str(x.as_str(), &y);
        println!("The longest string is: {}", z);
    } // y goes out of scope here
}
