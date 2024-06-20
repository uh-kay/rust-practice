use std::io;

fn main() {
    println!("Let's convert temperatures!");

    println!("Please input the unit you want to convert to. [C/F]");
    let mut convert_unit = String::new();

    io::stdin()
        .read_line(&mut convert_unit)
        .expect("Failed to recognize unit!");

    let temperature = convert_unit;

    println!("You want to convert to: {}", temperature);
    println!("Please input the amount.");
    let mut amount = String::new();

    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to recognize number.");

    let amount: f64 = match amount.trim().parse() {
        Ok(amount) => amount,
        Err(_) => {
            panic!("That wasn't a valid input! Temperatures can only be numbers!");
        }
    };

    match temperature.as_str() {
        "C\n" => println!("Result: {:.1}°C", ftoc(amount).round() as i32),
        "F\n" => println!("Result: {:.1}°F", ctof(amount).round() as i32),
        _ => println!("t = {:?}", temperature),
    }
}

fn ctof(c: f64) -> f64 {
    (c * 1.8) + 32.0
}

fn ftoc(f: f64) -> f64 {
    (f - 32.0) / 1.8
}