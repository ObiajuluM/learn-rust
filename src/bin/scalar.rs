#![allow(unused)] // make to ignore some warnings

fn main() {
    // Scalar types represent a single value. Common scalar types include: integers, floating-point numbers, booleans, and characters.

    // integer
    // Signed integers: i8, i16, i32, i64, i128, isize
    let i0: i8 = 0; // 8-bit signed integer  -2**(8-1) ~ 2**(8-1)-1  (-128 to 127)
    let i1: i16 = 1; // 16-bit signed integer -2**(16-1) ~ 2**(16-1)-1  (-32,768 to 32,767)
    let i2: i32 = 2; // 32-bit signed integer -2**(32-1) ~ 2**(32-1)-1  (-2,147,483,648 to 2,147,483,647)
    let i3: i64 = 3; // 64-bit signed integer -2**(64-1) ~ 2**(64-1)-1  (-9,223,372,036,854,775,808 to 9,223,372,036,854,775,807)
    let i4: i128 = 4; // 128-bit signed integer -2**(128-1) ~ 2**(128-1)-1
    let i5: isize = 5; // pointer-sized signed integer - depends on architecture (32 or 64 bits)

    // unsigned integers: u8, u16, u32, u64, u128, usize
    let u0: u8 = 0; // 8-bit unsigned integer 0 ~ 2**8-1  (0 to 255)
    let u1: u16 = 1; // 16-bit unsigned integer 0 ~ 2**16-1  (0 to 65,535)
    let u2: u32 = 2; // 32-bit unsigned integer 0 ~ 2**32-1  (0 to 4,294,967,295)
    let u3: u64 = 3; // 64-bit unsigned integer 0 ~ 2**64-1  (0 to 18,446,744,073,709,551,615)
    let u4: u128 = 4; // 128-bit unsigned integer 0 ~ 2**128-1
    let u5: usize = 5; // pointer-sized unsigned integer - depends on architecture (32 or 64 bits)

    // floating-point numbers
    let f1: f32 = 3.14; // 32-bit floating-point number
    let f2: f64 = 2.718281828459045; // 64-bit floating-point number

    // boolean
    let t: bool = true;
    let f: bool = false;

    // character - defined with single quotes
    let c: char = 'c';

    // Type conversion
    let i: i32 = 1;
    let u: u32 = i as u32;
    let x: u32 = u + (i as u32);

    // Min and Max values, sclar types that can be ordered have min and max values
    let min_i32 = std::i32::MIN;
    let max_i32 = std::i32::MAX;
    println!("Min i32: {}, Max i32: {}", min_i32, max_i32);

    let min_char: char = char::MIN;
    let max_char: char = char::MAX;
    println!("Min char: {}, Max char: {}", min_char, max_char);

    // Overflows
    let u: u32 = u32::MAX;
    // let overflowed = u + 1; // this will cause a compile-time error
}
