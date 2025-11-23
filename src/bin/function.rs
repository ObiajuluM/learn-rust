#![allow(unused)] // make to ignore some warnings

fn add(x: u32, y: u32) -> u32 {
    // return x + y; // explicit return
    x + y // implicit return
}

// functions that diverge
fn forever() -> ! {
    loop {
        println!("This function never returns!");
    }
}
fn crash() -> ! {
    panic!("This function always panics!");
}

fn print() {
    println!("Hello from print function");
}

fn main() {
    let x = 5;
    let y = 10;
    let result = add(x, y);
    println!("The sum of {} and {} is {} ", x, y, result);
    //
    print();

    // diverge
    forever();
    crash();
}
