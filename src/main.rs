use serde::Deserialize;
use reqwest::header::{HeaderMap, HeaderValue};
use std::io;
use std::error::Error as StdError;

#[derive(Deserialize, Debug)]
struct WeatherData {
    current: CurrentData,
}

#[derive(Deserialize, Debug)]
struct CurrentData {
    temp_c: f32,
}

#[derive(Debug)]
struct CustomError(String);

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for CustomError {}

async fn fetch_temperature(city: &str, api_key: &str) -> Result<f32, Box<dyn StdError>> {
    let url = format!("https://weatherapi-com.p.rapidapi.com/current.json?q={}", city);

    // Create a reqwest client and configure headers
    let mut headers = HeaderMap::new();
    headers.insert("X-RapidAPI-Host", HeaderValue::from_static("weatherapi-com.p.rapidapi.com"));
    headers.insert("X-RapidAPI-Key", HeaderValue::from_str(api_key).expect("Invalid API key"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let response_text = response.text().await?;
        let weather_response: WeatherData = serde_json::from_str(&response_text)?;
        let real_temperature = weather_response.current.temp_c;
        Ok(real_temperature)
    } else {
        println!("error in the api");
        Err(Box::new(CustomError("Error in the API".to_string())))
    }
}

fn greet_user() {
    let greeting: String = "Hello user, choose a city to know the weather".to_string();
    println!("{}", greeting);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let api_key = "";
    let mut input_user: String = String::new();
    greet_user();
    io::stdin()
        .read_line(&mut input_user)
        .expect("Error reading the input from the user");
    input_user = input_user.trim().to_string();
    match fetch_temperature(&input_user, api_key).await {
        Ok(temperature_data) => {
            println!("Temperature in {} : {}°C", input_user, temperature_data);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
    Ok(())
}
