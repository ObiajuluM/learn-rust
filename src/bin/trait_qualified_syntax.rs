#![allow(unused)] // make to ignore some warnings

trait Color {
    fn get(&self) -> String;
}

trait Rectangle {
    fn get(&self) -> (i32, i32, u32, u32);
}

struct Square {
    top: i32,
    left: i32,
    size: u32,
    color: String,
}

impl Color for Square {
    fn get(&self) -> String {
        // clone to return owned type
        self.color.clone()
    }
}

impl Rectangle for Square {
    fn get(&self) -> (i32, i32, u32, u32) {
        (self.top, self.left, self.size, self.size)
    }
}

fn main() {
    // Fully qualified syntax
    let square = Square {
        top: 10,
        left: 20,
        size: 30,
        color: "red".to_string(),
    };
    // square.get(); // ambiguous call becuase both traits color and rectangle have `get` method
    //  the fix is to use Fully qualified syntax, like so:
    let color = Color::get(&square); // specify which trait's method to call
    let rect = Rectangle::get(&square); // specify which trait's method to call
    println!("Square color: {}", color);
    println!("Square rect: {:?}", rect);
}
