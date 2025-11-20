#![allow(unused)] // make to ignore some warnings

use std::arch::x86_64::_SIDD_MASKED_POSITIVE_POLARITY;

// enum for color
#[derive(Debug, PartialEq)] // auto generate code to debug print enum variants, parialeq is to be able to compare enum variants

enum Color {
    Red,
    Green,
    Blue,
    Rgba(u8, u8, u8, f32),       // enum with parameters | tuple variant
    Hex(String),                 // enum with parameters | hexadecimal variant
    Hsl { h: u8, s: u8, l: u8 }, // enum with named fields | struct variant
}

fn main() {
    // Enum - short for enumeration, is a type that can be any one of several variants. [custom data to represent finite state]
    let color: Color = Color::Red;
    let color: Color = Color::Green;
    let color: Color = Color::Rgba(255, 0, 0, 1.0);
    let color: Color = Color::Hex(String::from("#FF0000"));
    let color: Color = Color::Hsl {
        h: 0,
        s: 100,
        l: 50,
    };
    println!("Color enum example {:?}", color);

    // Attributes - Debug and partial equal

    // Partial Eq
    println!("{}", Color::Red == Color::Red); // true
    println!("{}", Color::Red == Color::Green); // true

    // Option -  enum that can either be Some(value) or None
    let some_number: Option<i32> = Some(10);
    let no_number: Option<i32> = None;
    match some_number {
        Some(value) => println!("The number is: {}", value),
        None => println!("No number"),
    };

    // Result
}
