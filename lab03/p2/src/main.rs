fn check_addition(x: u32, y: u32) -> u32 {
    if x > std::u32::MAX - y {
        panic!("Addition overflow for u32");
    } else {
        return x + y;
    }
}

fn check_multiplication(x: u32, y: u32) -> u32 {
    if x > std::u32::MAX / y && x != 0 && y != 0 {
        panic!("Multiplication overflow for u32");
    } else {
        return x * y;
    }
}

fn main() {
    let x: u32 = 4294967295;
    let y: u32 = 2;
    let result_add = check_addition(x, y);
    println!("Addition = {result_add}");

    let result_mul = check_multiplication(x, y);
    println!("Multiplication = {result_mul}");
}
