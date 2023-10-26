enum MyError {
    NotAscii,
    NotDigit,
    NotHexDigit,
    NotLetter,
    NotPrintable,
}

fn is_ascii(ch: char) -> bool {
    return ch as u32 <= 127;
}

fn is_letter(ch: char) -> bool {
    return (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z');
}

fn is_digit(ch: char) -> bool {
    return ch >= '0' && ch <= '9';
}

fn is_hex_digit(ch: char) -> bool {
    return (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f') || (ch >= 'A' && ch <= 'F');
}

fn is_printable(ch: char) -> bool {
    match ch {
        '\u{0020}'..='\u{007E}' => true,
        _ => false,
    }
}

fn to_uppercase(ch: char) -> Result<char, MyError> {
    if is_letter(ch) {
        if ch >= 'A' && ch <= 'Z' {
            Ok(ch)
        } else {
            Ok((ch as u8 - 32 as u8) as char)
        }
    } else {
        Err(MyError::NotLetter)
    }
}

fn to_lowercase(ch: char) -> Result<char, MyError> {
    if is_letter(ch) {
        if ch >= 'a' && ch <= 'z' {
            Ok(ch)
        } else {
            Ok((ch as u8 + 32 as u8) as char)
        }
    } else {
        Err(MyError::NotLetter)
    }
}

fn print_char(ch: char) -> Result<(), MyError> {
    if is_printable(ch) {
        println!("{ch}");
        Ok(())
    } else {
        Err(MyError::NotPrintable)
    }
}

fn char_to_number(ch: char) -> Result<u32, MyError> {
    if !is_ascii(ch) {
        return Err(MyError::NotAscii);
    }
    if !is_digit(ch) {
        return Err(MyError::NotDigit);
    }
    Ok(ch as u32 - '0' as u32)
}

fn char_to_number_hex(ch: char) -> Result<u32, MyError> {
    if !is_ascii(ch) {
        return Err(MyError::NotAscii);
    }
    if !is_hex_digit(ch) {
        return Err(MyError::NotHexDigit);
    }
    if is_digit(ch) {
        Ok(ch as u32 - '0' as u32)
    } else if ch >= 'a' && ch <= 'z' {
        Ok(ch as u32 - 'a' as u32 + 10)
    } else {
        Ok(ch as u32 - 'A' as u32 + 10)
    }
}

fn print_error(error: MyError) {
    match error {
        MyError::NotAscii => println!("Character is not ASCII error!"),
        MyError::NotDigit => println!("Character is not a digit error!"),
        MyError::NotHexDigit => println!("Character is not a hexagonal digit error!"),
        MyError::NotLetter => println!("Character is not a letter error!"),
        MyError::NotPrintable => println!("Character is not printable error!"),
    }
}

fn main() {
    let ch: char = 'a';

    match to_uppercase(ch) {
        Ok(result) => println!("Uppercase = {result}"),
        Err(error) => print_error(error),
    }

    match to_lowercase(ch) {
        Ok(result) => println!("Lowercase = {result}"),
        Err(error) => print_error(error),
    }

    match print_char(ch) {
        Ok(()) => println!("Character printed"),
        Err(error) => print_error(error),
    }

    match char_to_number(ch) {
        Ok(result) => println!("Number = {result}"),
        Err(error) => print_error(error),
    }

    match char_to_number_hex(ch) {
        Ok(result) => println!("Hexagonal number = {result}"),
        Err(error) => print_error(error),
    }
}
