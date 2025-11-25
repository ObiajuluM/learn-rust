#![allow(unused)] // make to ignore some warnings

fn take(s: String) {
    // println!("Inside f(): {}", s);
}

fn copy(i: i32) {}

// Ownership rules in Rust:
// 1. Each value in Rust has a variable that's called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// primitive types like integers implement the Copy trait, so they are copied rather than moved.
// examples of primitive types: integers, floating-point numbers, booleans, and characters.

fn main() {
    // owner of "rust" is s
    let s = String::from("rust");

    // owner of -1 is i
    let i: i32 = -1;

    // there can only be one owner at a time

    let s = String::from("rust");
    // owner of "rust" is s1
    let s1 = s;

    println!("{s1}"); // s is no longer valid here

    let s2 = s1; // deep copy of the data
    println!(" {s2}");

    //  this compiles because the value is copied over
    let i: i32 = -1;
    let i1 = i; // shallow copy of the data
    println!("{i}, {i1}");

    // 3. When the owner goes out of scope, the value will be dropped.
    let s = String::from("rust");
    // moves s into the if block
    if (true) {
        s;
    } // s goes out of scope here and is dropped

    let s = String::from("rust");
    {
        let s1 = s;
    }

    let s = String::from("rust");
    take(s);

    let i = i32::from(-1);
    copy(i);
    println!("{i}");
}
