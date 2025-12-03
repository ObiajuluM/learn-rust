#![allow(unused)] // make to ignore some warnings

// Operator overloading with generics in rust
// Implementing the Add trait for a custom struct Point

use std::ops::Add;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// implement Add trait for Point struct
// trait Add<RHS = Self> {
//     type Output;

//     fn add(self, rhs: RHS) -> Self::Output;
// }
impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p0 = Point { x: 1.9, y: 2.5 };
    let p1 = Point { x: 3.9, y: 4.4 };
    let p2 = p0 + p1;
    println!("Point p2: {:?}", p2);
}
