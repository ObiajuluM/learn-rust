#![allow(unused)] // make to ignore some warnings

// attaches the Debug trait implementation to the MathError enum automatically. This lets you print enum values with println!("{:?}", value) for readable output during debugging.
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    Other,
}

fn div(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn main() {
    // error with panic
    // panic!("This is a panic error!");

    // Option error handling
    let arr = [1, 2, 3];
    // arr[9]; // this will cause a panic at runtime
    // Option<&i32> = Some(&value) or None || refernce to value or none
    let x: Option<&i32> = arr.get(9);
    match x {
        Some(v) => println!("Value found: {}", v),
        None => println!("No value found at this index"),
    }

    // Result error handling
    let x = 1;
    let y = 1;
    let z = div(x, y);
    match z {
        Ok(v) => println!("Result: {:?}", v),
        Err(e) => println!("Error: {:?}", e),
    };
}
