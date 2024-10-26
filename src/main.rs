use std::{fmt::format, io};
use serde::Deserialize;
use colored::*;

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
    let url: String = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&unit=metric&appid={}", city, country_code, api_key);
    let response = reqwest::blocking::get(url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json);
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
    )

    // function to get weather emojii according to the temperature
    fn get_temp_emoji(temp:  f64) -> &'static str {

        if temp < 0.0 {
            "🥶"
        }
    }



}