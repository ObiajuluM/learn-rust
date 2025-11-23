#![allow(unused)] // make to ignore some warnings

// modules in rust
// use pub to make module or items public

mod foo {
    pub fn print() {
        println!("Hello from foo module!");
    }
}

//  define a module
mod my {
    //  import foo into the module scope

    use super::foo; // super means go ane above the current module

    pub fn call_foo() {
        foo::print();
    }

    //  modules functions are private by default use `pub` to make them public
    pub fn greet() {
        f();
        println!("Hello from my_module!");
    }

    fn f() {
        println!("private function");
        a::print();
    }

    // nested module
    pub mod a {
        #[derive(Debug)]
        pub struct S {
            pub id: u32,
            pub name: String,
        }

        //
        pub fn print() {
            println!("a");
        }
    }
}

fn main() {
    // use module
    my::greet();

    my::a::print();

    let s = my::a::S {
        id: 1,
        name: String::from("example"),
    };
    println!("Struct S - id: {}, name: {}", s.id, s.name);
    println!("{:?}", s);
}
