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
    println!("all times are in EST");
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


fn draw_graph(times: &[String], temperatures: &[f32]) {
    let max_temperature = temperatures.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let min_temperature = temperatures.iter().cloned().fold(f32::INFINITY, f32::min);

    let temperature_range = max_temperature - min_temperature;
    let graph_height = 10;
    let temperature_per_height = temperature_range / graph_height as f32;

    println!("Temperature Graph");

    let max_time_width = times.iter().map(|time| time.len()).max().unwrap_or(0);

    for (time, temperature) in times.iter().zip(temperatures.iter()) {
        let height = ((temperature - min_temperature) / temperature_per_height) as usize;

        let padded_time = format!("{:<width$}", time, width = max_time_width);

        print!("{} | ", padded_time);

        for _ in 0..height {
            print!("█");
        }

        println!(" {:.1}°F", temperature);
    }
}

async fn get_weather(latitude: &str, longitude: &str) -> Result<(), Error> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m&temperature_unit=fahrenheit&forecast_days=3&timezone=America%2FNew_York",
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

    let formatted_times: Vec<String> = times
        .iter()
        .map(|time| {
            let date_parts: Vec<&str> = time.split('T').collect();
            let time_parts: Vec<&str> = date_parts[1].split(':').collect();

            let hour: i32 = time_parts[0].parse().unwrap();
            let minute: &str = time_parts[1];

            let formatted_hour = if hour == 0 {
                12
            } else if hour > 12 {
                hour - 12
            } else {
                hour
            };

            let am_pm = if hour < 12 {
                "AM"
            } else {
                "PM"
            };

            let day_month_parts: Vec<&str> = date_parts[0].split('-').collect();
            let month: &str = match day_month_parts[1] {
                "01" => "Jan",
                "02" => "Feb",
                "03" => "Mar",
                "04" => "Apr",
                "05" => "May",
                "06" => "Jun",
                "07" => "Jul",
                "08" => "Aug",
                "09" => "Sep",
                "10" => "Oct",
                "11" => "Nov",
                "12" => "Dec",
                _ => "",
            };
            let day: &str = day_month_parts[2];

            format!("{} {}, {}:{:02} {}", month, day, formatted_hour, minute, am_pm)
        })
        .collect();

    draw_graph(&formatted_times, &temperatures);

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