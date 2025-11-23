#![allow(unused)] // make to ignore some warnings

fn f1() -> Result<i32, String> {
    println!("Function f1 called");
    Ok(10)
}
fn f2() -> Result<i32, String> {
    println!("Function f2 called");
    Ok(10)
}

fn main() -> Result<(), String> {
    // let res = f1();
    // match res {
    //     Ok(v) => println!("f1 returned: {}", v),
    //     Err(e) => println!("f1 error: {}", e),
    // };

    //  question mark to unwrap value returned from f1 or return the error
    // the question mark can only be used in functions that return Result or Option types
    let res = f1()?;
    println!("f1 returned: {}", res);

    return Ok(());
}
