#![allow(unused)] // make to ignore some warnings

// Trait bound - specfies constraints on a generic type

// function with trait bound
// the geberic type T must implement both Display and Debug traits
fn f<T: std::fmt::Display + std::fmt::Debug>(t: T) {
    println!("Display: {}", t);
    println!("Debug: {:?}", t);
}

trait A {}
trait B {}
trait C {}

impl A for u32 {}
impl B for u32 {}
impl C for u32 {}
impl A for i32 {}

// here T must implement traits A
fn c<T: A>(x: T) {}
fn m<T: A + B>(x: T) {}

// alternative syntax using where clause
fn w<T, U>(x: T, y: U)
where
    T: A + B,
    U: B + C,
{
}

fn main() {
    let u: u32 = 1;
    let i: i32 = -1;
    let f: f32 = 1.0;

    c(u); // u32 implements trait A
    c(i); // i32 implements trait A
    // c(f); // error f32 does not implement trait A

    m(u);
    // m(i); // error i32 does not implement trait B
    // m(f); // error f32 does not implement trait A or B

    w(u, u);
    // w(u, i); // error i32 does not implement trait B
}
