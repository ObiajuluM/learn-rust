#![allow(unused)] // make to ignore some warnings

// Traits are like interfaces in other languages
// They define shared behavior that types can implement

struct Solidity {
    version: String,
}

struct Vyper {
    version: String,
}

//  define trait Compiler for Solidity and Vyper
// with method signature for compile
trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

trait Test {
    // default implementation
    fn test(&self, file_path: &str) -> String {
        format!("Testing {} with default test", file_path)
    }
}

// implement Compiler trait for Solidity
impl Compiler for Solidity {
    // self is a reference to the instance of the struct
    fn compile(&self, file_path: &str) -> String {
        format!(
            "Compiling {} with Solidity compiler version {}",
            file_path, self.version
        )
    }
}

// implement Test trait for Solidity
impl Test for Solidity {
    fn test(&self, file_path: &str) -> String {
        format!("Forge {} with Solidity {}", file_path, self.version)
    }
}

// implement Compiler trait for Vyper
impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!(
            "Compiling {} with Vyper compiler version {}",
            file_path, self.version
        )
    }
}

impl Test for Vyper {
    // use default implementation from the test trait by not defining overriding it here

    // fn test(&self, file_path: &str) -> String {
    //     format!("Vyper test {} with Vyper {}", file_path, self.version)
    // }
}

fn compile(lang: &impl Compiler, file_path: &str) -> String {
    // &impl Compiler means any type that implements the Compiler trait
    // call compile method on the passed language
    return lang.compile(file_path);
}

fn main() {
    let solc = Solidity {
        version: "0.8.0".to_string(),
    };

    let vyper = Vyper {
        version: "0.4".to_string(),
    };

    let solc_output = compile(&solc, "MyContract.sol");
    let vyper_output = compile(&vyper, "MyContract.vy");

    // call the tests method
    let solc_test = solc.test("MyContract.sol");
    let vyper_test = vyper.test("MyContract.vy");

    println!("{}", solc_output);
    println!("{}", vyper_output);
    println!("-------------------");
    println!("{}", solc_test);
    println!("{}", vyper_test);
}
