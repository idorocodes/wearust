# wearust

A Rust library for fetching **current weather and location data** from Weatherstack’s API. Built with **async support**, safe JSON deserialization using `serde`, and robust handling of extra fields.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wearust = "0.1.0"
tokio = { version = "1", features = ["full"] } # needed for async
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
anyhow = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```




```rust
use wearust::{find_location, find_weather};
use tokio;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Fetch location info
    let location_info = find_location("city","api_key").await?;
    println!("City: {}", location_info.location.name);
    println!("Country: {}", location_info.location.country);
    println!("Local time: {}", location_info.location.localtime);

    // Fetch current weather
    let weather = find_weather("city","api_key").await?;
    println!("Temperature: {}°C", weather.current.temperature);
    println!("Description: {}", weather.current.weather_descriptions.join(", "));
    
    Ok(())
}


