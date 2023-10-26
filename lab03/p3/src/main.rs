enum MyError {
    Overflow,
}

fn check_addition(x: u32, y: u32) -> Result<u32, MyError> {
    if x > std::u32::MAX - y {
        Err(MyError::Overflow)
    } else {
        Ok(x + y)
    }
}

fn check_multiplication(x: u32, y: u32) -> Result<u32, MyError> {
    if x > std::u32::MAX / y && x != 0 && y != 0 {
        Err(MyError::Overflow)
    } else {
        Ok(x * y)
    }
}

fn multiplicate_the_addition(a: u32, b: u32) -> Result<u32, MyError> {
    let result_add = check_addition(a, b)?;
    let result_mul = check_multiplication(result_add, 2)?;
    Ok(result_mul)
}

fn main() {
    let x: u32 = 4294967295;
    let y: u32 = 2;

    match multiplicate_the_addition(x, y) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => match error {
            MyError::Overflow => println!("Overflow error"),
        },
    }
}
