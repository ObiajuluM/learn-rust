#![allow(unused)] // make to ignore some warnings

// generic methods and associated functions in rust

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn move_to(&mut self, x: T, y: T) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    let mut p1 = Point::new(1, 2); // T inferred as i32
    println!("Point p1: {:?}", p1);

    p1.move_to(3, 4);
    println!("Moved Point p1: {:?}", p1);

    //
    let mut p2 = Point::new(1.0, 2.0); // T inferred as f64
    println!("Point p2: {:?}", p2);

    p2.move_to(3.5, 4.5);
    println!("Moved Point p2: {:?}", p2);
}
