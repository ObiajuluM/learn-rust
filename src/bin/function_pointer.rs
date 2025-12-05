#![allow(unused)] // make to ignore some warnings

//  Functions Pointers

fn add(x: u32, y: u32) -> u32 {
    x + y
}

// pass function pointers as parameters
fn do_twice(f: fn(u32, u32) -> u32, a: u32, b: u32) -> u32 {
    f(a, b) + f(a, b)
}

//
fn push(v: &mut Vec<u32>, x: u32) {
    v.push(x);
}

fn f_mut_twice(f: fn(&mut Vec<u32>, u32), v: &mut Vec<u32>, x: u32) {
    f(v, x);
    f(v, x);
}

fn main() {
    //  define a function pointer to a variable
    let f: fn(u32, u32) -> u32 = add;
    println!("f(1, 2) = {}", f(1, 2));

    // pass function pointer to another function
    let result = do_twice(add, 3, 4);
    println!("do_twice(add, 3, 4) = {}", result);

    // function pointer with mutable reference
    let mut v = vec![1, 2, 3];
    f_mut_twice(push, &mut v, 4);
    println!("v after f_mut_twice: {:?}", v);
}
