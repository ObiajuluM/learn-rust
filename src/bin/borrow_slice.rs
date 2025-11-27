#![allow(unused)] // make to ignore some warnings

// Slices are a reference to a portion in memory

fn borrow_slice(s: &[i32]) {
    println!("{:#?}", s);
}

fn borrow_mut(s: &mut [i32]) {
    s[0] = -1;
}

fn split_at(s: &[i32], i: usize) -> (&[i32], &[i32]) {
    /// Borrows a slice of `s` starting at index `i` and extending to the end.
    ///
    /// - Uses the range `i..` to create a right-hand slice.
    /// - The resulting reference is tied to the lifetime of `s` (no allocation).
    /// - Panics at runtime if `i` is out of bounds for `s`.
    let left = &s[..i];
    let right = &s[i..];
    (left, right)
}

fn main() {
    // immutable
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let s: &[i32] = &a[1..=4]; // slice from index 1 to 4
    borrow_slice(s);
    println!("{:?}", s);

    // mutable slice
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    let s: &mut [i32] = &mut a[0..=2]; // slice from index 0 to 2
    borrow_mut(s);
    println!("{:?}", s);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let (s0, s1) = split_at(&a, 2);
    println!("{:?} {:?}", s0, s1);
    println!("{:?}", a); // original array remains unchanged    s
}
