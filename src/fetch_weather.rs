use anyhow::Error;
use reqwest;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub current:  WeatherCondition,
}

#[derive(Debug, Deserialize)]
pub struct WeatherCondition {
    pub observation_time: String,
    pub temperature: i32,
    pub weather_code: i32,
    pub weather_icons: Vec<String>,
    pub weather_descriptions: Vec<String>,
    #[serde(flatten)]
    #[allow(dead_code)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}

pub async fn find_weather(location: &str,access_key:&str) -> Result<WeatherResponse, Error> {
   
    if location == "city" || location.len() == 0 {
        panic!("Location must be a city name");
    }

    if access_key == "access_key" || access_key.len() <= 0 {
        panic!("Access key must be a valid key");
    }
   
    let url = format!(
        "https://api.weatherstack.com/current?access_key={}&query={}",access_key,
        location.to_string(),
    );

    let res = reqwest::get(&url).await?;
    let weather_response: WeatherResponse = res.json().await?;

    Ok(weather_response)
}
     



    

