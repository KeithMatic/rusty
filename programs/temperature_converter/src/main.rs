use std::io;

const NINE_OVER_FIVE: f64 = 1.8;

fn main() {
    println!("Welcome to a temperature converter!");

    loop {
        println!("Which scale would you like to convert from? Fahrenheit (F) or Celsius (C)");

        let mut scale = String::new();

        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line for scale.");

        println!("Enter degrees value to convert.");

        let mut degrees = String::new();

        io::stdin()
            .read_line(&mut degrees)
            .expect("Failed to read line for degress.");

        let degrees: f64 = match degrees.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match scale.to_uppercase().as_str() {
            "F" => {
                let celsius = (degrees - 32f64) / NINE_OVER_FIVE;
                println!(
                    "{} degrees Fahrenheit is equal to {} degrees Celsius.",
                    degrees, celsius
                );
                break;
            }
            "C" => {
                let fahrenheit = (degrees * NINE_OVER_FIVE) + 32f64;
                println!(
                    "{} degrees Celsius is equal to {} degrees Fahrenheit.",
                    degrees, fahrenheit
                );
                break;
            }
            _ => println!("Invalid input, please enter F for Fahrenheit or C for Celsius"),
        }
    }
}
