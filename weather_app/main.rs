use std::io::{self, Write};
// the two endpoints for converting city to long/lat and then getting weather form long/lat
// https://geocode.maps.co/search?q={stockholm%20sweden}
// https://api.open-meteo.com/v1/forecast?latitude=59.33&longitude=18.07&hourly=temperature_2m&temperature_unit=fahrenheit&forecast_days=3&timezone=Europe%2FBerlin

fn main() {
    println!("RUST WEATHER CLI");
    println!("Stockholm Sweden Weather Forecast");
    let city = get_user_input("Enter your city: ");
    println!("{}", city)

}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}