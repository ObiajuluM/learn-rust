#![allow(unused)] // make to ignore some warnings

fn main() {
    //  Tuple types group multiple values into one compound type. The values can be of different types.
    let tup: (i32, f64, char, bool) = (500, 6.4, 'a', false);
    let (x, y, z, b) = tup; // destructuring

    //  destructuring
    println!("The value of x, y, z, b is: {}, {}, {}, {}", x, y, z, b);

    // empty tuple, aka unit type
    let t = ();

    //  nested tuple
    let nested_tup: ((i32, f64), char, bool) = ((1, 2.0), 'b', true);

    // accessing tuple elements by index
    let first_element = nested_tup.0;
    let second_element = nested_tup.1;
    println!(
        "First element: {:?}, Second element: {}",
        first_element, second_element
    );
    // access nested tuple
    let nested_first = nested_tup.0.0;
    let nested_second = nested_tup.0.1;
    println!(
        "Nested first: {}, Nested second: {}",
        nested_first, nested_second
    );

    // Arrays and slices - fixed-size list where all elements have the same type.
}
