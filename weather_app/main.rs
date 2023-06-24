use std::io::{self, Write};
use reqwest::Error;
use serde::{Deserialize, Serialize};

// the two endpoints for converting city to long/lat and then getting weather from long/lat
// https://geocode.maps.co/search?q={stockholm%20sweden}
// https://api.open-meteo.com/v1/forecast?latitude=59.33&longitude=18.07&hourly=temperature_2m&temperature_unit=fahrenheit&forecast_days=3&timezone=Europe%2FBerlin


#[derive(Debug, Deserialize, Serialize)]
struct Response {
    #[serde(rename = "lat")]
    latitude: String,
    #[serde(rename = "lon")]
    longitude: String,
}

#[tokio::main]
async fn main() {
    println!("RUST WEATHER CLI");
    println!("Stockholm Sweden Weather Forecast");

    let city = get_input("Enter your city and country/state: ");
    if let Err(err) = get_coordinates(&city).await {
        eprintln!("Error: {}", err);
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_lowercase()
}

#[derive(Debug, Deserialize)]
struct HourlyWeather {
    time: Vec<String>,
    temperature_2m: Vec<f32>,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    latitude: f32,
    longitude: f32,
    hourly: HourlyWeather,
}

async fn get_weather(latitude: &str, longitude: &str) -> Result<(), Error> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m&temperature_unit=fahrenheit&forecast_days=3&timezone=Europe%2FBerlin",
        latitude,
        longitude
    );

    let response = reqwest::get(&url).await?;
    let body = response.text().await?;

    let parsed_response: WeatherResponse = match serde_json::from_str(&body) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("Failed to parse weather response: {}", err);
            return Ok(());
        }
    };

    let times = parsed_response.hourly.time;
    let temperatures = parsed_response.hourly.temperature_2m;

    for (time, temperature) in times.into_iter().zip(temperatures.into_iter()) {
        println!("Time: {}, Temperature: {}°F", time, temperature);
    }

    Ok(())
}

async fn get_coordinates(city: &str) -> Result<(), Error> {
    let url = format!("https://geocode.maps.co/search?q={}", city);
    let response = reqwest::get(&url).await?;
    let body = response.text().await?;

    let parsed_response: Vec<Response> = match serde_json::from_str(&body) {
        Ok(parsed) => parsed,
        Err(err) => {
            eprintln!("Failed to parse coordinates response: {}", err);
            return Ok(());
        }
    };

    if let Some(result) = parsed_response.get(0) {
        let latitude = &result.latitude;
        let longitude = &result.longitude;
        println!("Latitude: {}", latitude);
        println!("Longitude: {}", longitude);

        get_weather(latitude, longitude).await?;
    } else {
        eprintln!("No coordinates found for the given city.");
    }

    Ok(())
}