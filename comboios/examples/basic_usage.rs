//! Basic usage example for the Comboios API
//!
//! This example demonstrates how to use the new ComboiosApi struct
//! to search for stations, get timetables, and retrieve train details.
//!
//! Run with: cargo run --example basic_usage

use comboios::ComboiosApi;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for better logging
    tracing_subscriber::fmt::init();

    println!("Comboios API Example");
    println!("====================\n");

    // Create API client with default settings
    let api = ComboiosApi::new();

    // Example 1: Search for stations
    println!("1. Searching for stations containing 'Lisboa'...");
    let stations = match api.get_stations("Lisboa").await {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to search stations: {}", e);
            return Ok(());
        }
    };

    println!("Found {} stations:", stations.response.len());
    for (i, station) in stations.response.iter().take(3).enumerate() {
        println!(
            "   {}. {} (ID: {})",
            i + 1,
            station.designation,
            station.code
        );
    }

    // Example 2: Get timetable for first station
    let first_station = match stations.response.first() {
        Some(s) => s,
        None => {
            println!("No stations found");
            return Ok(());
        }
    };

    println!("\n2. Getting timetable for '{}'...", first_station.designation);
    let timetable = match api.get_station_timetable(&first_station.code).await {
        Ok(t) => t,
        Err(e) => {
            println!("Failed to get timetable: {}", e);
            return Ok(());
        }
    };

    println!("Found {} trains:", timetable.len());
    for (i, train) in timetable.iter().take(3).enumerate() {
        println!(
            "   {}. Train {} from {} to {}",
            i + 1,
            train.train_number,
            train.train_origin.designation,
            train.train_destination.designation
        );
        if let Some(departure) = &train.departure_time {
            println!("      Departure: {}", departure);
        }
    }

    // Example 3: Get train details
    let first_train = match timetable.first() {
        Some(t) => t,
        None => {
            println!("No trains found");
            return Ok(());
        }
    };

    println!("\n3. Getting details for train {}...", first_train.train_number);
    match api.get_train_details(first_train.train_number as u16).await {
        Ok(train_details) => {
            println!("Train {} details:", train_details.train_number);
            if let Some(first_stop) = train_details.stops.first()
                && let Some(departure) = &first_stop.departure_time
            {
                println!("   First departure: {}", departure);
            }
            if let Some(last_stop) = train_details.stops.last()
                && let Some(arrival) = &last_stop.arrival_time
            {
                println!("   Final arrival: {}", arrival);
            }
            println!("   Total stops: {}", train_details.stops.len());
        }
        Err(e) => println!("Failed to get train details: {}", e),
    }

    println!("\n4. Example with custom configuration...");

    // Create API client with custom configuration
    let custom_client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .user_agent("ComboiosExample/1.0")
        .build()?;

    let custom_api = ComboiosApi::with_client(custom_client).with_timeout(Duration::from_secs(15));

    match custom_api.get_stations("Porto").await {
        Ok(stations) => {
            println!(
                "Found {} Porto stations with custom client",
                stations.response.len()
            );
        }
        Err(e) => println!("Custom client request failed: {}", e),
    }

    println!("\nExample completed!");
    Ok(())
}
