#![allow(unused)] // make to ignore some warnings

// structs

// create a struct 1
#[derive(Debug)] // to be able to debug print the struct
struct Point {
    x: f32,
    y: f32,
}

// create a struct 2 - tuple struct
struct Point3d(f32, f32, f32);

// empty struct
struct Empty;

// nested struct
#[derive(Debug)] // to be able to debug print the struct
struct Circle {
    center: Point,
    radius: u32,
}

fn main() {
    // create
    let p: Point = Point { x: 1.0, y: 2.0 };
    println!("Point struct example x: {}, y: {}", p.x, p.y);

    // point 3d
    let p3d: Point3d = Point3d(1.0, 2.0, 3.0);
    println!(
        "Point3d struct example x: {}, y: {}, z: {}",
        p3d.0, p3d.1, p3d.2
    ); // p3d.notation to access the struct fields 

    // empty struct
    let e: Empty = Empty;
    println!("Empty struct example {:?}", std::any::type_name::<Empty>());

    // circle
    let c: Circle = Circle {
        center: Point { x: 1.0, y: 2.0 },
        radius: 1,
    };
    println!(
        "Circle struct example center: ({}, {}), radius: {}",
        c.center.x, c.center.y, c.radius
    );
    // debug print
    println!("Circle struct debug print example {:?}", c);

    //  shortcut
    let x = 1.0;
    let y = 2.0;
    let p_shortcut = Point { x, y }; // shorthand syntax when variable names match
    println!(
        "Point struct shortcut example x: {}, y: {}",
        p_shortcut.x, p_shortcut.y
    );

    //  copy fields
    let p0 = Point { x: 5.0, y: 10.0 };
    let p1 = Point { x: 2.0, ..p0 }; //  copy the rest of the fields from p0
    println!(
        "Point struct copy fields example p1 x: {}, y: {}",
        p1.x, p1.y
    );

    // update struct without '+='
    let mut p = Point { x: 0.0, y: 0.0 };
    p.x = p.x + 3.0;
    p.y += 4.0;
    println!("Point struct update example x: {}, y: {}", p.x, p.y);
}
