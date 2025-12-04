#![allow(unused)] // make to ignore some warnings
//  More Iterator adaptors [Functions] in Rust

use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // Iterator adaptor:
    // map, filter, collect, zip, fold
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    // map - applies a function to each item and returns a new iterator
    let mut data: Vec<u32> = Vec::new();
    for v in vals {
        data.push(v * 2);
    }

    // Alternatively, we can use map directly
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    let data: HashSet<u32> = vals.iter().map(|x| x * 2).collect();
    println!("Mapped Values To Hashset: {:?}", data);

    // Filter - filters items based on a predicate function
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    //  filter gives a reference
    let even_vals: Vec<u32> = vals.into_iter().filter(|x| x % 2 == 0).collect();
    println!("Filtered Even Values: {:?}", even_vals);

    // Zip - combines two iterators into a single iterator of pairs
    let nums = vec![1, 2, 3];
    let chars = vec!['a', 'b', 'c'];
    let zipped: Vec<(i32, char)> = nums.into_iter().zip(chars.into_iter()).collect();
    println!("Zipped Values: {:?}", zipped);

    // convert zipped into a hashmap
    let nums = vec![1, 2, 3];
    let chars = vec!['a', 'b', 'c'];
    let zipped_hashmap: HashMap<i32, char> = nums.into_iter().zip(chars.into_iter()).collect();
    println!("Zipped Values: {:?}", zipped_hashmap);

    // Fold - reduces the iterator to a single value by applying a binary operation
    // the single value reduced to is called accumulator
    let vals: Vec<u32> = vec![1, 2, 3, 4];
    let sum: u32 = vals.into_iter().fold(0, |acc: u32, x| acc + x);
    println!("Folded Sum: {}", sum);
}
