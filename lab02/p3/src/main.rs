fn add_space(s: &mut String, mut n: i32) {
    while n > 0 {
        s.push(' ');
        n -= 1;
    }
}

fn add_str(s: &mut String, x: &str) {
    s.push_str(&x);
}

fn reverse(mut n: i32) -> i32 {
    let mut new_n: i32 = 0;
    while n > 0 {
        new_n = new_n * 10 + n % 10;
        n /= 10;
    }
    return new_n;
}

fn add_integer(s: &mut String, n: i32) {
    let mut reverse_n: i32 = reverse(n);
    let mut counter: i32 = 0;

    while reverse_n > 0 {
        counter += 1;
        if counter % 3 == 1 && counter != 1 {
            s.push('_');
        }
        let digit: u8 = (reverse_n % 10) as u8;
        s.push((b'0' + digit) as char);
        reverse_n /= 10;
    }
}

fn add_float(s: &mut String, f: f32, decimals: i32) {
    let integer_part = f.trunc();
    let mut reverse_int_part: i32 = reverse(integer_part as i32);

    while reverse_int_part > 0 {
        let digit: u8 = (reverse_int_part % 10) as u8;
        s.push((b'0' + digit) as char);
        reverse_int_part /= 10;
    }
    s.push_str(".");

    let mut fractional_part: f32 = f - integer_part as f32;
    let mut counter: i32 = 0;
    while counter < decimals {
        let digit: u8 = (fractional_part * 10.0).trunc() as u8;
        s.push((b'0' + digit) as char);
        fractional_part = (fractional_part * 10.0) - digit as f32;
        counter += 1;
    }
}

fn main() {
    let s = &mut String::from("");
    add_space(s, 42);
    add_str(s, "I");
    add_space(s, 1);
    add_str(s, "💚\n");
    add_space(s, 42);
    add_str(s, "RUST.\n\n");
    add_space(s, 5);
    add_str(s, "Most");
    add_space(s, 12);
    add_str(s, "create");
    add_space(s, 6);
    add_integer(s, 306437968);
    add_space(s, 11);
    add_str(s, "and");
    add_space(s, 5);
    add_str(s, "lastest");
    add_space(s, 9);
    add_str(s, "is\n");
    add_space(s, 10);
    add_str(s, "downloaded");
    add_space(s, 9);
    add_str(s, "has");
    add_space(s, 13);
    add_str(s, "downloads");
    add_space(s, 5);
    add_str(s, "the");
    add_space(s, 9);
    add_str(s, "version");
    add_space(s, 4);
    add_float(s, 2.038, 3);
    add_str(s, ".\n");

    println!("{}", s);
}
