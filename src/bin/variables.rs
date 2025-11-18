#![allow(unused)] // make to ignore some warnings

fn main() {
    //  variables are immutable by default
    let x: i32 = -123;

    // mutable variable
    let mut y: i32 = 456;
    y += 789;

    // rust may infer the type
    let z = -123;

    // constant variable
    const MAX_POINTS: u32 = 100_000;

    // shadowing / redeclaration of variable
    let x: i32 = -5;
    let x: bool = true;

    // vector Vec<_> rust infers the type
    let v: Vec<_> = vec![1, 2, 3, 4, 5];
}
