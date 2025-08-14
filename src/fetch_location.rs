use anyhow::Error;
use reqwest;
use serde::Deserialize;
use chrono::NaiveDateTime;

#[derive(Debug, Deserialize)]
pub struct LocationResponse {
    pub location:  CityLocation,
}

#[derive(Debug, Deserialize)]
pub struct CityLocation {
    pub name: String,
    pub country: String,
    pub region: String,
    pub lat: String,
    pub lon: String,
    pub timezone_id: String,
    #[serde(deserialize_with = "naive_datetime_from_str")]
    pub localtime: NaiveDateTime,
    pub localtime_epoch: i128,
    pub utc_offset: String,
}



fn naive_datetime_from_str<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M")
        .map_err(serde::de::Error::custom)
}

pub async fn find_location(location: &str, access_key:&str) -> Result<LocationResponse, Error> {
   
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
    let location_response: LocationResponse = res.json().await?;

    Ok(location_response)
}
