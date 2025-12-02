#![allow(unused)] // make to ignore some warnings

// Trait Sized - used to indicate that a type has a known size at compile time
// Most types in Rust are Sized by default, but some types like slices and trait objects are not.
// Neccesary for allocating values on the stack
// By default, generic types are assumed to be Sized unless specified otherwise.

// ?Sized - indicates that a type may or may not be Sized
// Used in generic type parameters to allow for unsized types
// Commonly used with references and smart pointers, which can handle unsized types

fn f<T: Sized>(x: T) {}
fn g<T: ?Sized>(x: &T) {
    // function body
}

fn main() {
    //Sized
    let i = 10; // i32 is Sized
    let x: f64 = 20.5; // f64 is Sized
    let b: bool = true; // bool is Sized

    f(i);
    f(x);
    f(b);
}
