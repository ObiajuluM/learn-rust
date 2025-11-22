#![allow(unused)] // make to ignore some warnings

fn main() {
    let mut i = 0;
    loop {
        println!("loop {i}");
        if i == 5 {
            break; // to prevent infinite loop
        }
        i += 1;
    }

    // while
    let mut i = 0;
    while i < 3 {
        println!("while: {i}");
        i += 1;
    }

    // for loop
    // with `=` for inclusive range
    for i in 0..=5 {
        println!("for: {i}");
    }

    // iterating over arrays
    let arr = [10, 20, 30, 40, 50];
    for a in arr {
        println!("array value: {a}");
    }

    // ussize and range
    let arr = [100, 200, 300, 400, 500];
    // loop through indices
    for i in 0..arr.len() {
        println!("array index {i} has value {}", arr[i]);
    }

    // for loop vectors
    let vec = vec![1, 2, 3];
    // can do once, recommend to user `.iter()` or &vec
    // for v in vec {
    for v in vec.iter() {
        println!("vector value: {v}");
    }
    for v in vec.iter() {
        println!("vector value: {v}");
    }

    // return value from loop
    let mut i = 0;
    let z = loop {
        println!("loop {i}");
        if i == 3 {
            break 99; // to prevent infinite loop break and return value
        }
        i += 1;
    };
    println!("returned from loop: {z}");

    // labels
    'outer: for i in 0..5 {
        'inner: for j in 0..5 {
            println!("i: {i}, j: {j}");
            if i == 1 && j == 2 {
                break 'outer; // breaks outer loop
            }
        }
    }
}
