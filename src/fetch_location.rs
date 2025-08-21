use anyhow::Error;
use chrono::NaiveDateTime;
use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationResponse {
    pub location: CityLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityLocation {
    pub name: String,
    pub country: String,
    pub region: String,
    pub lat: f64,
    pub lon: f64,
    #[serde(deserialize_with = "naive_datetime_from_str")]
    pub localtime: NaiveDateTime,
    pub localtime_epoch: i128,
    pub tz_id: String,
}

fn naive_datetime_from_str<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M").map_err(serde::de::Error::custom)
}

pub async fn find_location(location:&str, access_key:&str) -> Result<LocationResponse, Error> {
    if location == "city" || location.is_empty() {
        panic!("Error : Location must be a string and must not be empty!");
    }

    if access_key == "access_key" || access_key.is_empty() {
       panic!("Error : Access_key must be a string and must not be empty!, get it from weatherapi.com");
    }
        
    

    let url = format!(
        "https://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no",
        access_key,location
       
    );

    let res = reqwest::get(&url).await?;
    let location_response:LocationResponse= res.json().await?;

    Ok(location_response)
}
