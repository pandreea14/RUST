fn calculate_area(length: f64, width: f64) -> Option<f64> {
    if length > 0.0 && width > 0.0 {
        Some(length * width)
    } else {
        None
    }
}

fn calculate_volume(length: f64, width: f64, height: f64) -> Result<f64, &'static str> {
    if length > 0.0 && width > 0.0 && height > 0.0 {
        Ok(length * width * height)
    } else {
        Err("Invalid dimensions.")
    }
}

fn main() {
    let length = 5.0;
    let width = 3.0;

    let area = calculate_area(length, width);

    match area {
        Some(area) => println!("Area of the rectangle: {area}"),
        None => println!("Invalid dimensions. Area cannot be calculated."),
    }

    let height = -7.2;
    let volume = calculate_volume(length, width, height);

    match volume {
        Ok(result) => println!("Volume of the rectangle: {result}"),
        Err(_) => println!("Invalid dimensions. Volume cannot be calculated."),
    }
}
