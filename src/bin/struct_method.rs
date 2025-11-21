#![allow(unused)] // make to ignore some warnings

// struct methods

#[derive(Debug)] // to be able to debug print the struct
struct Point {
    x: f32,
    y: f32,
}

// struct methods

// associated function (like a constructor) - operate on the the type
// impl, struct type
impl Point {
    // static method - operates on the type itself, not an instance
    fn zero() -> Self {
        // use Self to refer to the type Point
        Self { x: 0.0, y: 0.0 }
    }

    // method - operate on the instance of the type
    fn move_to(&mut self, x: f32, y: f32) {
        // &mut self to modify the instance [mutable reference to self 'Point']
        self.x = x;
        self.y = y;
    }

    fn dist(&self) -> f32 {
        // &self to read the instance [immutable reference to self 'Point']
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

fn main() {
    let mut p = Point::zero(); // call associated function
    println!("Point zero: {:?}", p);

    p.move_to(10.0, 20.0); // call method to move the point
    println!("Point moved to: {:?}", p);

    let distance = p.dist(); // call method to get distance from origin
    println!("Distance from origin: {}", distance);
}
