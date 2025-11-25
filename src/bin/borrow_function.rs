#![allow(unused)] // make to ignore some warnings
// Borrow and function

fn take(s: &String) {
    println!("Inside take(): {}", s);
}

fn main() {
    // Take Ownership
    let s = String::from("rust");
    take(s); // borrow s by passing a reference to it

    // println!("{s}"); // s is still valid here
}
