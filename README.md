# ☁ wearust

A Rust library for fetching current weather and location data from WeatherAPI.
It uses async Rust with Tokio, safe JSON parsing with serde, and hides away the messy API boilerplate so you just call a function and get typed data back.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wearust = "0.1.0"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```


### USAGE

```rust
use wearust::{find_location, find_weather};
use tokio;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let api_key = "your_api_key"; //get it from weatherapi.com
    let city_name = "your_prefered_city"

    // Fetch location info
    let location_info = find_location(city_name, api_key).await?;
    println!("City name : {}", location_info.location.name);
    println!("Country : {}", location_info.location.country);
    println!("Region : {}", location_info.location.region);
    println!("Latitude : {}", location_info.location.lat);
    println!("Longitude : {}", location_info.location.lon);
    println!("Date : {}", location_info.location.localtime.date());
    println!("Time : {}", location_info.location.localtime.time());

    // Fetch current weather
    let weather = find_weather("Lagos", api_key).await?;
    println!("Last Updated: {}", weather.current.last_updated);
    println!("Temperature in Celsius : {}°C", weather.current.temp_c);
    println!("Temperature in Fahrenheit : {}°F", weather.current.temp_f);
    println!("Weather Condition : {}", weather.current.condition.text);
    println!("Weather Icon : {}", weather.current.condition.icon);

    Ok(())
}

```

