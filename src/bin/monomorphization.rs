#![allow(unused)] // make to ignore some warnings

// Monomorphization in Rust: the process of generating concrete implementations of generic code for specific types

// in simple terms, it will copy and paste the code using generics and replace the generic types with concrete types

struct Point<T> {
    x: T,
    y: T,
}

//  when you instanties Point with u32 or i32 types, rust creates two concrete structs that look like this:
// struct Pointi32 {
//     x: i32,
//     y: i32,
// }
// struct Pointu32 {
//     x: u32,
//     y: u32,
// }
/// same shit happens with functions

fn main() {
    let p0: Point<u32> = Point { x: 10, y: 20 }; // T is u32
    let p1: Point<i32> = Point { x: 1, y: 2 }; // T is i32
}
