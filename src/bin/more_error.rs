#![allow(unused)] // make to ignore some warnings

// attaches the Debug trait implementation to the MathError enum automatically. This lets you print enum values with println!("{:?}", value) for readable output during debugging.
#[derive(Debug)]
enum MathError {
    DivisionByZero,
}

#[derive(Debug)]
enum ParseError {
    InvalidInt,
}

// Format trait is used to define how the type should be formatted when printed. Here, we provide a custom implementation for MathError to display a user-friendly message.
impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "math error {:?}", self)
    }
}
// Format trait is used to define how the type should be formatted when printed. Here, we provide a custom implementation for ParseError to display a user-friendly message.
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error {:?}", self)
    }
}

impl std::error::Error for MathError {}
impl std::error::Error for ParseError {}

fn f1() -> Result<u32, MathError> {
    Err(MathError::DivisionByZero)
}
fn f2() -> Result<u32, ParseError> {
    Err(ParseError::InvalidInt)
}

// Box<dyn std::error::Error> is a trait object that can hold any error type that implements `Error`. This allows f3 to return different error types from f1 and f2 while conforming to a single return type.
fn f3() -> Result<u32, Box<dyn std::error::Error>> {
    let n1 = f1()?;
    let n2 = f2()?;
    Ok(n1 + n2)
}

fn main() {
    let z = f3();
    println!("Result: {:?}", z);
}
