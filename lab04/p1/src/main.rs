use std::{fs, io};

fn read_file() -> Result<(), io::Error> {
    let s = fs::read_to_string(r"C:\Users\Andreea\OneDrive\Desktop\RUST\lab04\p1\src\input.txt")?;

    let mut maxi_chars = 0;
    let mut maxi_bytes = 0;
    let mut copy_line_bytes: &str = "";
    let mut copy_line_chars: &str = "";

    for line in s.lines() {
        let bytes_count = line.len();
        if bytes_count > maxi_bytes {
            maxi_bytes = bytes_count;
            copy_line_bytes = line;
        }

        let mut chars_count = 0;
        for _i in line.chars() {
            chars_count += 1;
        }
        if chars_count > maxi_chars {
            maxi_chars = chars_count;
            copy_line_chars = line;
        }
    }

    println!("Longest line by the number of bytes is: {copy_line_bytes}");
    println!("Longest line by the number of chars is: {copy_line_chars}");

    fs::write("input.txt", &s)?;

    Ok(())
}

fn main() {
    if let Err(_) = read_file() {
        println!("Error!!");
    }
}
