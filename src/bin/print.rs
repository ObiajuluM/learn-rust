#![allow(unused)] // make to ignore some warnings

#[derive(Debug)] // declare attributes for struct, meta data to add functionality to struct
struct Lang {
    language: String,
    version: i32,
}

fn main() {
    let lang = "rust";
    println!("{} is awesome!", lang);
    println!("{lang} is awesome!");

    // positional arguments
    let x = 2;
    println!("{} + {} = {}", x, x, x + x);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // print structs
    let rust = Lang {
        language: String::from("Rust"),
        version: 2024,
    };
    println!("{:?}", rust); // this will give an error if Lang does not implement Debug trait
    println!("{:#?}", rust); // with line breaks for better readability

    // run `cargo fmt` to format code
}
