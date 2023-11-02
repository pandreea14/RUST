use std::{fs, io};

fn rot13() -> Result<(), io::Error> {
    let s = fs::read_to_string(r"C:\Users\Andreea\OneDrive\Desktop\RUST\lab04\p2\src\input.txt")?;

    for chars in s.chars() {
        if chars.is_ascii() == true {
            if chars.is_lowercase() == true {
                if chars >= 'a' && chars < 'n' {
                    let aux = (chars as u8 + 13) as char;
                    print!("{aux}");
                } else {
                    let aux = (chars as u8 - 13) as char;
                    print!("{aux}");
                }
            } else {
                if chars >= 'A' && chars < 'N' {
                    let aux = (chars as u8 + 13) as char;
                    print!("{aux}");
                } else {
                    let aux = (chars as u8 - 13) as char;
                    print!("{aux}");
                }
            }
        } else {
            panic!("Eroare caracterul nu e ASCII!");
        }
    }

    fs::write("input.txt", &s)?;

    Ok(())
}

fn main() {
    if let Err(_) = rot13() {
        println!("Error!");
    }
}
