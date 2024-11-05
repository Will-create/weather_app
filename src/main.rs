use std::io;
use reqwest;
use serde::Deserialize;
use colored::*;
use dotenv::dotenv;
use std::env;


// Create a Struc to help deserialize the json from openweather api
#[derive(Deserialize, Debug)]
struct WeatherResponse {
weather: Vec<Weather>,
main: Main,
wind: Wind,
name: String
}

// A struct to represent the weather description

#[derive(Deserialize,Debug)]
struct Weather {
description: String
}

// struct to represent the main weather parameters
#[derive(Deserialize, Debug)]
struct Main {
temp: f64,
humidity: f64,
pressure: f64
}

// Struct to represent the wind informations
#[derive(Deserialize, Debug)]
struct Wind {
speed: f64
}

// Create a function to get the weather informations
fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error>{
let url: String = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}", city, country_code, api_key);
let response = reqwest::blocking::get(url)?;
let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
// println!("{:?}", response_json);
Ok(response_json)
}

// Create a function to display the weather informations
fn display_weather_info(response: &WeatherResponse) {

// extract weather informations
let description: &String = &response.weather[0].description;
let temperature: f64 = response.main.temp;
let humidity: f64 = response.main.humidity;
let pressure: f64 = response.main.pressure;
let wind_speed: f64 = response.wind.speed;

let weather_text: String = format!(
    "
    Weather in {}: {} {}
    > Temperature: {:.1}°C
    > Humidity: {:.1}%
    > Pressure: {:.1}hPa
    > Wind Speed: {:.1} m/s
    ",
    response.name,
    description,
    get_temp_emoji(temperature),
    temperature,
    humidity,
    pressure,
    wind_speed
);

// coloring the weather text based on the weather conditions
let weather_text_colored: ColoredString = match description.as_str() {
    "clear sky" => weather_text.bright_yellow(),
    "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
    "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
    "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
    _ => weather_text.normal()
};

// pint the weather informations

println!("{}", weather_text_colored);

}


// function to get weather emojii according to the temperature
fn get_temp_emoji(temp:  f64) -> &'static str {

    if temp < 0.0 {
        "🥶"
    } else if temp >= 0.0 && temp < 10.0 {
        "🌩️"
    } else if temp >= 10.0 && temp < 20.0 {
        "🌥️"
    } else if temp >= 20.0 && temp < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

fn main() {
    dotenv().ok();
    println!("{}", "Welcome to Weather Station!".bright_yellow());

    loop {

        // Read the city
        println!("{}", "Please enter the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input!");
        let city = city.trim();

        // Read the country code
        println!("{}", "Please enter the Country code:".bright_green());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input!");
        let country_code = country_code.trim();

        let api_key = env::var("API_KEY").expect("API_KEY in .env must be set");

        match get_weather_info(&city, &country_code, &api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprintln!("Error {}", err);
            }
        }

        println!("{}", "Do you want to search for weather in inother city? (yes/no):".bright_green());
        let mut input = String::new();
        io::stdin()
.read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for ysing our software!");
            break;
        }
    }
}
