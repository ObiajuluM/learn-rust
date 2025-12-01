#![allow(unused)] // make to ignore some warnings

// Generic traits in rust

// generic trait
trait List<T> {
    fn count(&self) -> usize;
    fn first(&self) -> &T;
}

// implement generic trait 'List' for a tuple where the types are u32
impl List<u32> for (u32, u32) {
    fn count(&self) -> usize {
        2
    }

    fn first(&self) -> &u32 {
        &self.0
    }
}

//  implement 'List' for vector
impl<T> List<T> for Vec<T> {
    fn count(&self) -> usize {
        self.len()
    }

    fn first(&self) -> &T {
        &self[0]
    }
}

impl<X, Y> List<(X, Y)> for [(X, Y); 2] {
    fn count(&self) -> usize {
        2
    }

    fn first(&self) -> &(X, Y) {
        &self[0]
    }
}

fn main() {
    let xy: (u32, u32) = (10, 20);
    println!("Tuple count: {}", xy.count());
    println!("Tuple first: {}", xy.first());

    //
    let arr: [(u32, &str); 2] = [(1, "2"), (3, "4")];
    println!("Array count: {}", arr.count());
    println!("Array first: {:?}", arr.first());
}
