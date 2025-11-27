#![allow(unused)] // make to ignore some warnings

// Dereferencing is accessing the value a reference points to

use std::num::ParseIntError;

fn modify(s: &mut String) {
    // does this take ownership? No, it borrows mutably
    *s += "!"; // dereference to modify the original value
}

fn main() {
    let mut s = String::from("hello");
    let s1 = &mut s;

    //
    *s1 += "?"; // dereference to modify the original value
    println!("{s}");

    let mut s = String::from("hello");
    modify(&mut s); // pass a mutable reference
    println!("{s}"); // original string is modified

    // Deref coercion
    // Automatically dereferences in some situations
    let x = 1;
    let y = &x;
    let z = &x;
    let w = *y + *z;
    println!("w = {w}"); // prints 2
}
