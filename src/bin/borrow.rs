#![allow(unused)] // make to ignore some warnings

fn main() {
    let s = String::from("hello");
    let s1 = &s; // no problem
    let s2 = &s; // no problem

    // Borrow  - temporary use of a value without taking ownership
    // To borrow a value in Rust, you use references, which are indicated by the & symbol.
    // Immutable borrow - cannot modify the value
    // Mutable borrow - allows modification of the value

    // immutable borrow - multiple immutable borrows allowed
    let s = String::from("hello");
    let s1 = &s; // immutable borrow
    let s2 = &s; // immutable borrow
    let s3 = s2; // immutable borrow

    println!("s1: {}", s3); // OK

    // Mutable borrow - only one mutable borrow allowed at a time
    let mut s = String::from("hello");
    let s0 = &mut s; // mutable borrow // OK will work because after this line s0 is not used
    let s1 = &mut s; // mutable borrow

    // let s2 = &mut s; // mutable borrow - ERROR: cannot borrow `s` as mutable more than once at a time

    s1.push_str("string");
    println!("s: {}", s); // OK

    // Cannot create a mutable refernce and immutable refernces at the same time
    let mut s = String::from("rust");
    let s1 = &s; // immutable borrow
    let s2 = &s; // immutable borrow
    // let s3 = &mut s; // mutable borrow - ERROR: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{}, {},", s1, s2);

    // A reference must not outlive the value it points to
    let s = String::from("hello");
    let s1 = &s; // r1 borrows s   
    // {
    //     let s1 = s;
    // } // s1 is dropped

    // drop a reference explicitly using std::mem::drop
    // std::mem::drop(s);

    println!("s1: {}", s1); // OK
}

// fn f(s: String) -> &String {
//     &s
// }
