fn prime(x: i32) -> bool {
    if x % 2 == 0 && x >= 2 || x < 2 {
        return false;
    }
    let mut ind: i32 = 3;
    while ind <= x / 2 {
        if x % ind == 0 {
            return false;
        }
        ind += 2;
    }
    return true;
}

fn main() {
    let mut nr: i32 = 0;
    while nr <= 100 {
        if prime(nr) == true {
            println!("nr: {nr}");
        }
        nr += 1;
    }
}
