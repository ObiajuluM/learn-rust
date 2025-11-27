#![allow(unused)] // make to ignore some warnings
// Borrow and function

fn take(s: &String) {
    println!("Inside take(): {}", s);
}
fn borrow(s: &str) {
    println!("Inside borrow(): {}", s);
}
fn borrow_mut(s: &mut String) {
    s.push_str(" is awesome");
    
}

fn main() {
    // Take Ownership
    let s = String::from("rust");
    take(&s); // borrow s by passing a reference to it

    // Borrow immutable -> doesnt move ownership
    let s = String::from("rust");
    borrow(&s);
    println!("{s}");

    // Borrow mutable -> doesnt move ownership
    let mut s = String::from("rust");
    borrow_mut(&mut s);
    println!("{s}");


    // modify a function in 3 steps
    // 1. take ownership 
    // 2. returns ownership
    // 3. borrows
}
