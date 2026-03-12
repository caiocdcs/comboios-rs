//! Basic usage example for the Comboios API
//!
//! This example demonstrates how to use the ComboiosApi struct
//! to search for stations and get departure/arrival boards.
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

    // Example 2: Get departure board for first station
    let first_station = match stations.response.first() {
        Some(s) => s,
        None => {
            println!("No stations found");
            return Ok(());
        }
    };

    let now = chrono::Local::now();
    let start = now.format("%Y-%m-%d %H:%M").to_string();
    let end = (now + chrono::Duration::hours(12)).format("%Y-%m-%d %H:%M").to_string();

    println!("\n2. Getting departures for '{}'...", first_station.designation);
    let boards = match api.get_station_timetable(&first_station.code, &start, &end).await {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to get timetable: {}", e);
            return Ok(());
        }
    };

    let total_trains: usize = boards.iter().map(|b| b.trains.len()).sum();
    println!("Found {} train entries:", total_trains);

    // Show trains from first board
    if let Some(board) = boards.first() {
        println!("\n   Board: {} ({} entries)", board.station_name, board.trains.len());
        
        for (i, train) in board.trains.iter().take(5).enumerate() {
            print!(
                "   {}. Train {}: {} to {} at {}",
                i + 1,
                train.train_number,
                train.origin_station_name,
                train.destination_station_name,
                train.time
            );
            if let Some(delay) = train.delay_minutes() {
                print!(" (delayed {} min)", delay);
            }
            println!();
            println!("      Service: {}", train.service_type);
            if !train.observations.is_empty() {
                println!("      Note: {}", train.observations);
            }
        }
    }

    // Example 3: Using convenience method
    println!("\n3. Using get_station_board_now() convenience method...");
    match api.get_station_board_now(&first_station.code).await {
        Ok(boards) => {
            let total: usize = boards.iter().map(|b| b.trains.len()).sum();
            println!("   Found {} upcoming trains", total);
        }
        Err(e) => println!("   Failed: {}", e),
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
