#![allow(unused)] // make to ignore some warnings

fn main() {
    // math operators
    let a: i32 = 1;
    let b: i32 = 2;
    let sum = a + b;
    let diff = b - a;
    let prod = a * b;
    let division = a / b; // integer division, rounds off [ removes decimal ]
    let rem = b % a; // this is not modulus, but remainder, can return a signed value
    println!(
        "Math operators example: sum: {}, diff: {}, prod: {}, division: {}, rem: {}",
        sum, diff, prod, division, rem
    );

    // literals
    let a = 1i32;
    let b = 2.5f64;
    let c = 1.2e5f32;
    // unnderscores in literals for readability
    let d = 1_000_000u32;
    println!("Literals example: a: {}, b: {}, c: {}, d: {}", a, b, c, d);

    // boolean operators
    let t: bool = true;
    let f: bool = false;
    let and = t && f;
    let or = t || f;
    let not = !t;
    println!(
        "Boolean operators example: and: {}, or: {}, not: {}",
        and, or, not
    );

    //  bitwise operators
    // 101
    let a: u8 = 5;
    // 011
    let b: u8 = 3;
    // and operation 101 & 011 = 001
    let and = a & b;
    // or operation 101 | 011 = 111
    let or = a | b;
    // xor operation 101 ^ 011 = 110
    let xor = a ^ b;
    // not operation ~101 = 010
    let not = !a;
    // left shift 101 << 1 = 1010
    let left_shift = a << 1;
    // right shift 101 >> 1 = 010
    let right_shift = a >> 1;
    println!(
        "Bitwise operators example: and: {}, or: {}, xor: {}, not: {}, left_shift: {:03b}, right_shift: {}",
        and, or, xor, not, left_shift, right_shift
    );
}
