use std::io::{self, Write};

fn main() {
    println!("RUST WEATHER CLI");
    println!("Stockholm Sweden Weather Forecast");
    let city = get_user_input("Enter your city: ");
    println!("{}", city)

}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}