#![allow(unused)] // make to ignore some warnings

// Super traits are traits that require other traits to be implemented first

trait Language {
    fn name(&self) -> String;
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;
}

//  the trait called CompiledLanguage is a super trait
// it implements both Language and Compiler traits - like multi inheritance
// thus any struct that wants to implement CompiledLanguage must also implement Language and Compiler
trait CompiledLanguage: Language + Compiler {
    fn execute(&self, file_path: &str) -> String {
        let compiled = self.compile(file_path);
        format!("Running compiled code of {}: {}", self.name(), compiled)
    }
}

// type that implements CompiledLanguage
struct Rust;

impl Language for Rust {
    fn name(&self) -> String {
        "Rust".to_string()
    }
}

impl Compiler for Rust {
    fn compile(&self, file_path: &str) -> String {
        format!("Compiling {} to binary", file_path)
    }
}

// make rust inmplement CompiledLanguage
impl CompiledLanguage for Rust {}

fn main() {
    // Super trait
    let rust = Rust;
    println!("{}", rust.execute("file_path"));
    println!("Language: {}", rust.name());
}
