#![allow(unused)] // make to ignore some warnings

// Generic iterators with associated types in Rust
// Implementing a iterator trait  for a custom type

fn main() {
    let vals: Vec<i32> = vec![1, 2, 3];
    //  iter - borrows and returns an iterator over reference &T
    // into_iter - takes ownership and returns an iterator that yields elements by value T -- runs for loop once
    // for v in vals { // implicitly calls into_iter()
    // iter_mut - borrows mutably and returns an iterator that yields mutable references &mut T
    for v in vals.iter() {
        println!("Value: {}", v);
    }

    // to modify values, we can use into_iter()
    for mut v in vals.into_iter() {
        v += 1;
        println!("Value: {}", v);
    }

    // Iter_mut example
    let mut vals_mut: Vec<i32> = vec![1, 2, 3];
    for v in vals_mut.iter_mut() {
        *v += 10; // dereference to modify the value
        println!("Mut Value: {}", v);
    }

    // iterators can be chained with other iterator adaptors like map, filter, etc.
    let squared: Vec<i32> = vec![1, 2, 3, 4].into_iter().map(|x| x * x).collect();
    println!("Squared Values: {:?}", squared);
}
