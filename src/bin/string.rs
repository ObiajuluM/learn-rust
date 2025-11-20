#![allow(unused)] // make to ignore some warnings

fn main() {
    // String and &str types - String is a growable, heap-allocated data structure [vector of u8 Vec<u8> valid utf 8],
    // &str is an immutable reference to a string slice.

    // when to use string
    // String -> mutate or data needs to be owned
    // &str -> read only

    // create string
    let mut s = String::from("Hello Rust 🦀");
    // get length
    let len: usize = s.len();
    println!("s: {s}");
    println!("len: {len}");

    // append a string slice
    s.push_str("world!");
    println!("{}", s);

    // str - string slice
    // &str
    // usually used str with refernce (borrowed)
    // immutable

    let mut msg = String::from("Hello Rust 🦀");
    // create string slice from string
    let s: &str = &msg[..4];
    let len: usize = s.len();
    println!("s: {s}");
    println!("len: {len}");

    // string slice from string literal
    // stored in binary
    // slice pointing to specific part in binary
    // immutable, because hard coded in binary
    let s: &str = "Hello Rust 🦀";
    let len: usize = s.len();
    println!("s: {s}");
    println!("len: {len}");

    //multi line string literal
    let multi_line: &str = r#"Hello"#;
    println!("multi_line: {multi_line}");

    let multi_line_json: &str = r#"
    {  "name": "John",
       "age": 30,
       "city": "New York"
    }
    "#;
    println!("multi_line_json: {multi_line_json}");

    // Deref coercion
    // convert a refernce of one type to another type
    let s: String = String::from("Hello Rust 🦀");
    // &String to &str
    let s_slice: &str = &s;
    println!("s_slice: {s_slice}");

    // add &str to String
    let mut s = "Hello Rust 🦀".to_string();
    s += "!";
    println!("s: {s}");

    // String interpolation
    let name: &str = "Rustacean";
    let emotion: char = '🦀';
    let greeting: String = format!("Hello, {} {emotion}!", name);
    println!("greeting: {greeting}");
}
