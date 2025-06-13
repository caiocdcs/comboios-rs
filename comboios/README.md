# Comboios Core

Core library for accessing Portuguese train (CP - Comboios de Portugal) data and schedules.

## Overview

This is the foundational library that provides domain models, HTTP client functionality, and error handling for interacting with Portuguese train APIs. This crate was built as part of learning Rust and serves as the backbone for all other components in the CP-PT Rust project.

## Features

- Domain models for stations, trains, and timetables
- Async HTTP client with error handling and timeouts
- JSON deserialization with flexible type handling
- Comprehensive error types
- Logging support with tracing

## Data Sources

- CP (Comboios de Portugal) - https://www.cp.pt/sites/spring
- IP (Infraestruturas de Portugal) - https://www.infraestruturasdeportugal.pt

## Usage

Add to your Cargo.toml:

```toml
[dependencies]
cp-pt = { path = "path/to/cp-pt" }
reqwest = "0.12"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Example

```rust
use cp_pt::client::{get_stations, get_station_timetable, get_train_details};
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // Search for stations
    let stations = get_stations(client.clone(), "Lisboa").await?;
    println!("Found {} stations", stations.data.len());

    // Get timetable for a station
    if let Some(station) = stations.data.first() {
        let timetable = get_station_timetable(client.clone(), &station.node_id).await?;
        println!("Found {} departures", timetable.len());

        // Get details for a specific train
        if let Some(departure) = timetable.first() {
            let train = get_train_details(client, departure.train_number).await?;
            println!("Train {} goes to {}", train.number, train.destination);
        }
    }

    Ok(())
}
```

## API Functions

### get_stations(client, station_name)

Search for stations by name.

Parameters:
- `client: reqwest::Client` - HTTP client instance
- `station_name: &str` - Station name to search for

Returns: `Result<StationResponse, CoreError>`

Example:
```rust
let stations = get_stations(client, "Porto").await?;
```

### get_station_timetable(client, station_id)

Get departure/arrival timetable for a station.

Parameters:
- `client: reqwest::Client` - HTTP client instance
- `station_id: &str` - Station ID from station search

Returns: `Result<Vec<Timetable>, CoreError>`

Example:
```rust
let timetable = get_station_timetable(client, "94001").await?;
```

### get_train_details(client, train_id)

Get detailed information about a specific train.

Parameters:
- `client: reqwest::Client` - HTTP client instance
- `train_id: u16` - Train number

Returns: `Result<Train, CoreError>`

Example:
```rust
let train = get_train_details(client, 123).await?;
```

## Error Handling

The library uses a custom `CoreError` type for HTTP and parsing errors:

```rust
use cp_pt::error::CoreError;

match get_stations(client, "InvalidStation").await {
    Ok(stations) => println!("Found stations: {:?}", stations),
    Err(CoreError::HttpError(e)) => eprintln!("Network error: {}", e),
    Err(CoreError::ParseError(e)) => eprintln!("Parsing error: {}", e),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Dependencies

- reqwest - Async HTTP client
- serde - JSON serialization/deserialization
- serde_json - JSON support
- anyhow - Error handling
- thiserror - Custom error types
- tracing - Structured logging

## Development

```bash
# Build
cargo build -p cp-pt

# Run tests
cargo test -p cp-pt

# Check types
cargo check -p cp-pt
```
