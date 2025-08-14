use std::io;

fn main() {
    println!("Temperature Converter");
    println!("Choose conversion:");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice = choice.trim();

    if choice == "1" {
        // Fahrenheit to Celsius
        println!("Enter temperature in Fahrenheit:");
        let mut fahrenheit = String::new();
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read input");

        let fahrenheit: f64 = fahrenheit.trim().parse().expect("Invalid number");

        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        println!("{fahrenheit}°F is {celsius:.2}°C");
    } else if choice == "2" {
        // Celsius to Fahrenheit
        println!("Enter temperature in Celsius:");
        let mut celsius = String::new();
        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read input");

        let celsius: f64 = celsius.trim().parse().expect("Invalid number");

        let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
        println!("{celsius}°C is {fahrenheit:.2}°F");
    } else {
        println!("Invalid option selected.");
    }
}
