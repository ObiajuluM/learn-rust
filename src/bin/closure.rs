#![allow(unused)] // make to ignore some warnings

// Closures in Rust
// Closures are anonymous functions that can capture variables in their environment.
// They are often used for short-lived operations or when you need to pass a function as an argument.

use std::vec;

fn main() {
    fn add(x: u32, y: u32) -> u32 {
        x + y
    }
    // Define a closure that adds two numbers to replace the above function
    let f = |x: u32, y: u32| -> u32 { x + y };
    // a second time
    let f = |x: u32, y: u32| -> u32 { x + y };

    // without type annotations (type inference)
    let f = |a, b| a + b;
    let result = f(3, 4);
    println!("The sum of 3 and 4 is: {}", result);

    // multiline closure
    let f = |x, y| {
        let sum = x + y;
        sum * 2
    };
    let result = f(3, 4);
    println!("The result of the multiline closure is: {}", result);

    //
    let v = 1;
    let f = |x: u32| x + v; // closure captures v from the environment
    let result = f(5);
    println!("The result of the closure capturing v is: {}", result);

    //  with map
    let w = vec![1, 2, 3];
    let w2: Vec<i32> = w.iter().map(|x| x + 1).collect();
}
