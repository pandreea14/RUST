fn coprimes(mut x: i32, mut y: i32) -> bool {
    if x == y || x == 0 || y == 0 {
        return false;
    }
    while x != y {
        if x > y {
            x = x - y;
        } else {
            y = y - x;
        }
    }
    if x == 1 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let mut index1: i32 = 0;
    while index1 <= 100 {
        let mut index2: i32 = 0;
        while index2 <= 100 {
            if coprimes(index1, index2) == true {
                println!("{index1} si {index2} sunt coprime");
            }
            index2 += 1;
        }
        index1 += 1;
    }
}
