#![allow(unused)] // make to ignore some warnings

fn main() {
    // let x: Option<i32> = None;
    let x: Option<i32> = Some(2);
    match x {
        Some(value) => println!("The value is: {}", value),
        _ => {}
    }
    // if let to replace the above match statement
    if let Some(value) = x {
        // if let only matches the Some variant
        println!("If Let value is: {}", value);
    }

    // let else
    let Some(v) = x else {
        // handle the None case,
        // diverge (panic or return)
        panic!("x is none");
    };

    println!("V is equal to {}", v);
}
