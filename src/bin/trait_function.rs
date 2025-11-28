#![allow(unused)] // make to ignore some warnings

// How to use traits as function outputs and inputs


trait Animal {
    fn speak(&self) -> String;
}

struct Cat;
struct Dog;

impl Animal for Cat {
    fn speak(&self) -> String {
        "Meow!".to_string()
    }
}
impl Animal for Dog {
    fn speak(&self) -> String {
        "Woof!".to_string()
    }
}

// static example - trait known at compile time aka // static dispatch, the trait is known at compile time
// trait as function parameter(input)
// prefix with impl
fn greet(animal: &impl Animal) -> String {
    format!("The static animal says: {}", animal.speak())
}

//  static return trait from function (output)
fn return_concrete_type() -> impl Animal {
    Dog
}

// dynamic example - trait known at runtime aka // dynamic dispatch, the trait is known at runtime
// prefix with dyn
fn greet_dyn(animal: &dyn Animal) -> String {
    return format!("The dynamic animal says: {}", animal.speak());
}

//  dynamic return trait from function (output)
//  use boxes
fn rand_animal(rand: u32) -> Box<dyn Animal> {
    if rand <= 10 {
        // Box::new(value) stores the value on the heap
        Box::new(Dog)
    } else {
        Box::new(Cat)
    }
}

fn main() {
    let cat = Cat;
    let dog = Dog;

    // static
    println!("{}", greet(&cat));
    println!("{}", greet(&dog));
    // static function print
    let animal = return_concrete_type();
    println!("The returned animal says: {}", animal.speak());

    // dynamic
    let animal_str = "cat";
    // use &dyn [trait] to tell rust it's a dynamic trait object
    let animal: &dyn Animal = match animal_str {
        "dog" => &Dog,
        _ => &Cat,
    };
    println!("{}", greet_dyn(animal));

    // Dynamic return trait from function
    let random_animal = rand_animal(5);
    println!("Random animal says: {}", random_animal.speak());
}
