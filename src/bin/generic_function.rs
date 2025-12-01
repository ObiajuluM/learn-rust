#![allow(unused)] // make to ignore some warnings

use std::cmp::PartialOrd;
// Generic Functions in Rust: functions that can operate on different data types
// T is a generic type parameter

//  function resctircted to u32 type only
fn swap(t: (u32, u32)) -> (u32, u32) {
    (t.1, t.0)
}

// generic function that can work with any type A, B
fn generic_swap<A, B>(t: (A, B)) -> (B, A) {
    (t.1, t.0)
}

/// Returns the largest element in a slice of T values.
/// If the slice is empty, returns None.
/// std::cmp::PartialOrd is a trait that allows comparison operators on generic types T
fn max<T: PartialOrd>(s: &[T]) -> Option<&T> {
    if s.len() == 0 {
        return None;
    }

    let mut largest = &s[0];
    for item in s.iter() {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

fn main() {
    let t = (1, 2);
    let s = swap(t);
    println!("Original pair: {:?}", t);
    println!("Swapped pair: {:?}", s);

    let t2 = ("hello", 3.14);
    let s2 = generic_swap(t2);
    println!("Original generic pair: {:?}", t2);
    println!("Swapped generic pair: {:?}", s2);

    //
    let numbers = vec![34, 50, 25, 100, 65];
    match max(&numbers) {
        Some(max_value) => println!("The largest number is {}", max_value),
        None => println!("The slice is empty."),
    }

    // test characters
    let chars = vec!['a', 'b', 'c', 'd'];
    match max(&chars) {
        Some(max_value) => println!("The largest character is {}", max_value),
        None => println!("The slice is empty."),
    }
}
