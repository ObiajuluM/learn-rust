#![allow(unused)] // make to ignore some warnings

fn main() {
    // Arrays and slices - fixed-size list where all elements have the same type.
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // array of 5 i32 elements
    // slice length is not known at compile time
    println!("arr[2] = {}", arr[2]); // accessing first element

    //  make an array modifiable
    let mut mut_arr: [i32; 3] = [10, 20, 30];
    mut_arr[0] = 100; // modifying first element
    println!("mut_arr[0] = {}", mut_arr[0]);

    let arr: [u32; 10] = [0; 10]; // array of 10 u32 elements initialized to 0
    println!("arr: {:?}", arr);

    // Slice - segments of an array
    let nums: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice: &[i32] = &nums[2..5]; // slice from index 2 to 4
    // & means get the reference to ...
    println!("slice: {:?}", slice);

    let last_elements: &[i32] = &nums[7..]; // slice from index 7 to end
    println!("last_elements: {:?}", last_elements);

    // all elements
    let all_elements: &[i32] = &nums[..]; // slice of the entire array
    println!("all_elements: {:?}", all_elements);
}
