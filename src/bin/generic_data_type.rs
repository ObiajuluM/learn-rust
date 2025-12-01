#![allow(unused)] // make to ignore some warnings

// Generic data types in Rust: primitive and compound
// T is a generic type parameter

enum Option<T> {
    // <T> is a generic type parameter
    Some(T),
    None,
}

enum Result<T, E> {
    // <T, E> are generic type parameters
    Ok(T),
    Err(E),
}

// struct Point<T>{ // generic struct
// struct Point<T=u32>{ // generic struct with default type u32 make sure the params are of type T too
struct Point<T = u32> {
    x: T,
    y: T,
}

fn main() {
    // some generic data types
    let x: Option<u32> = Option::Some(1);
    let y: Option<i32> = Option::Some(-1);

    let res: Result<bool, String> = Result::Ok(true);

    // vectors - generic collection // Vec<_> tells compiler to infer type
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    let p0 = Point { x: 10, y: 20 }; // T inferred as u32
    let p1: Point<f64> = Point { x: 1.0, y: 2.0 }; // T specified as f64

    //
    //
    // LESSON ENDS ABOVE THIS LINE, THE REST IS AI GENERATED EXAMPLES
    //
    //
    //

    // primitive data types
    let a: i32 = 10; // 32-bit integer
    let b: f64 = 20.5; // 64-bit floating point
    let c: bool = true; // boolean
    let d: char = 'R'; // character

    // compound data types
    let tup: (i32, f64, char) = (500, 6.4, 'A'); // tuple
    let (x, y, z) = tup; // destructuring tuple
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // array

    // print values
    println!("Integer: {}", a);
    println!("Float: {}", b);
    println!("Boolean: {}", c);
    println!("Character: {}", d);
    println!("Tuple: ({}, {}, {})", x, y, z);
    println!("Array: {:?}", arr);
}
