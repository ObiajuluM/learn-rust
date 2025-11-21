#![allow(unused)] // make to ignore some warnings

fn main() {
    let x: i32 = 10;

    if x % 2 == 0 {
        println!("{} is even", x);
    } else {
        println!("{} is odd", x);
    }

    // use to return a value, like a ternary operator
    let z: i32 = if x > 5 { x * 2 } else { x + 2 };
    println!("The value of z is: {}", z);
}
