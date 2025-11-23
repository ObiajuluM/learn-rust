#![allow(unused)] // make to ignore some warnings

fn main() {
    // error with panic
    // panic!("This is a panic error!");

    // Option error handling
    let x: Option<i32> = None;
    // match x {
    //     Some(v) => println!("Value found: {}", v),
    //     None => println!("No value found at this index"),
    // }

    let v = x.unwrap(); // unwrap will extract the value inside the Some variant, but will panic if it's None
    println!("Unwrapped value is: {}", v);

    let v = x.expect("x is none"); // expect will extract the value inside the Some variant, but will panic with a custom message if it's None
    println!("Unwrapped value is: {}", v);

    // Result error handling
    let x = 1;
    let y = 1;
    let z: Result<u32, String> = Ok(x / y);

    let v = z.unwrap(); // unwrap will extract the value inside the Ok variant, but will panic if it's Err
    println!("Unwrapped result is: {}", v);
}
