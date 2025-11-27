#![allow(unused)] // make to ignore some warnings

// Vectors are resizable arrays stored on the heap
// Vectors are a generic type

fn main() {
    //  Vec<T>
    let v: Vec<i32> = vec![-1, 0, 1];
    let v: Vec<u32> = vec![1, 2, 3];
    let v: Vec<i32> = Vec::new();
    let v = vec![1u8, 2, 3]; // type inferred
    let v = vec![1u8; 5]; // vector of 5 elements initialized to 1u8
    println!("{:?}, {:?}", v, v.len());

    // Accessing elements - get
    let v = vec![1, 2, 3];
    let x = v[0]; // direct indexing, panics if out of bounds
    let x = v.get(99); // returns Option<&T>
    // match x
    match x {
        Some(value) => println!("First element is {}", value),
        None => println!("No first element"),
    }

    // Updating elements
    let mut v = vec![1, 2, 3];
    v[0] = 10; // direct index and update

    // push : add element to the end
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    // Pop : remove the last element
    let mut v = vec![1, 2, 3];
    match v.pop() {
        Some(value) => println!("Popped {}", value),
        None => println!("Vector is empty"),
    }
    // match v.pop() {
    //     Some(value) => println!("Popped {}", value),
    //     None => println!("Vector is empty"),
    // }
    // match v.pop() {
    //     Some(value) => println!("Popped {}", value),
    //     None => println!("Vector is empty"),
    // }
    // match v.pop() {
    //     Some(value) => println!("Popped {}", value),
    //     None => println!("Vector is empty"),
    // }
    println!("{:?}", v);

    // Slice
    let v = vec![1, 2, 3, 4, 5];
    let slice: &[i32] = &v[1..4]; // slice from index 1 to 3
    println!("slice: {:?}", slice);
}
