use anyhow::Error;
use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  WeatherResponse {
   pub current: WeatherCondition ,
  
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherCondition {
    pub last_updated: String,
    pub temp_c: f64,
    pub temp_f: f64,
    pub condition: ConditionFields,
    #[serde(flatten)]
    #[allow(dead_code)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionFields {
    pub text: String,
    pub icon: String,
    #[serde(flatten)]
    #[allow(dead_code)]
    extra: HashMap<String, serde_json::Value>,
}



pub async fn find_weather(location:&str,access_key:&str) -> Result<WeatherResponse, Error> {
    if location == "city" || location.is_empty() {
        panic!("Location must be a city name");
    }

    if access_key == "access_key" || access_key.is_empty() {
        panic!("Access key must be a valid key");
    }

    let url = format!(
        "https://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
        access_key,location
       
    );


    let res = reqwest::get(&url).await?;
    let weather_response: WeatherResponse = res.json().await?;
    Ok(weather_response)
}
