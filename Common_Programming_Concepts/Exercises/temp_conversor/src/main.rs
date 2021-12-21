use std::io;
fn main() {
    let mut temperature = String::new();
    let mut choice = String::new();
    println!("1- Celsius -> Fahrenheit\n2- Fahrenheit -> Celsius");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read from stdin");
    let choice: u8 = choice.trim().parse().unwrap();

    io::stdin()
        .read_line(&mut temperature)
        .expect("failed to read from stdin");

    let temperature: f32 = temperature.trim().parse().unwrap();
    if choice == 1 {
        println!("{}", celsius_fahrenheit(temperature))
    } else if choice == 2 {
        println!("{}", fahrenheit_celsius(temperature))
    }
    
}

fn celsius_fahrenheit(x: f32) -> f32 {
    let fahrenheit = (x * (9.0/5.0)) + 32.0;
    fahrenheit
}

fn fahrenheit_celsius(x: f32) -> f32 {
    let celsius = (x - 32.0) * (5.0/9.0);
    celsius
}