#![allow(unused)] // make to ignore some warnings

enum Animal {
    Cat,
    Dog,
    Mouse,
    Duck,
}

fn main() {
    let x = 3i32;

    match x {
        1 => println!("x is one"),
        2 => println!("x is two"),
        // 3 or 4 or 5
        3 | 4 | 5 => println!("x is between three and five"),
        6..=10 => println!("x is between six and ten"),
        // catch all case
        _ => println!("x is something else"),
    }

    match x {
        // print the value matched from the range
        i @ 1..=10 => println!("x matched range: {i}"),
        _ => println!("x is something else"),
    }

    // return value from match
    let animal = Animal::Cat;
    let sound = match animal {
        Animal::Cat => "meow",
        Animal::Dog => "woof",
        Animal::Mouse => "squeak",
        Animal::Duck => "quack",
        _ => "?",
    };
    println!("Animal sound: {}", sound);

    // Option
    let maybe_number: Option<i32> = Some(42);
    match maybe_number {
        Some(n) => n,
        None => 0,
    };

    // Result
    let result: Result<i32, String> = Ok(10);
    match result {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {}", e);
            0
        }
    };
}
