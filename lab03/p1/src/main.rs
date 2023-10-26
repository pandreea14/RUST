fn is_prime(x: u16) -> bool {
    if x % 2 == 0 && x > 2 || x < 2 {
        return false;
    }

    let mut ind: u16 = 3;
    while ind <= x / 2 {
        if x % ind == 0 {
            return false;
        }
        ind += 2;
    }
    return true;
}

fn next_prime(x: u16) -> Option<u16> {
    let mut value: u16 = x + 1;
    let mut prime: bool = false;
    while value < std::u16::MAX && !prime {
        if is_prime(value) {
            prime = true;
        }
        value += 1;
    }

    if prime {
        Some(value - 1)
    } else {
        None
    }
}

fn main() {
    let mut x = 6500;
    while let Some(i) = next_prime(x) {
        println!("next_prime is {i}");
        x = i as u16;
    }
}
