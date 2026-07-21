use std::io;

fn main() {
    println!("Temperature Converter");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let choice = choice.trim();

    if choice == "1" {
        let mut input = String::new();
        println!("Enter Celsius:");

        io::stdin().read_line(&mut input).unwrap();

        let celsius: f64 = input.trim().parse().unwrap();

        let fahrenheit = celsius * 9.0 / 5.0 + 32.0;

        println!("{:.2}°F", fahrenheit);

    } else if choice == "2" {

        let mut input = String::new();

        println!("Enter Fahrenheit:");

        io::stdin().read_line(&mut input).unwrap();

        let fahrenheit: f64 = input.trim().parse().unwrap();

        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

        println!("{:.2}°C", celsius);

    } else {

        println!("Invalid choice.");

    }
}
