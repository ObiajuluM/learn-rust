#![allow(unused)] // make to ignore some warnings

// From Trait and Into Trait
// They are used to convert between types in Rust.

// Into<T> for U: Defines how to convert a value of type U into type T. It is implemented on the source type.

// From<T> for U: Defines how to convert a value of type T into type U. It is implemented on the destination type.

// implement From trait for custom struct
#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

// implement from trait for tuple of type u32 to point struct
// once to implement the From trait, the Into trait is automatically implemented
impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Point {
            x: value.0,
            y: value.1,
        }
    }
}

fn main() {
    // From convert from tuple to point
    // define tuple
    let t: (u32, u32) = (10, 20);
    // convert tuple to point struct using from trait
    let p1: Point = Point::from(t);
    println!("Point from From trait: {:?}", p1);

    // From convert from point to tuple
    let p: Point = t.into();
    println!("Point from Into trait: {:?}", p);
}
