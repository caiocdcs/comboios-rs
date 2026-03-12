use comboios::ComboiosApi;

// Integration tests requiring network access
// Run with: cargo test -- --ignored

#[tokio::test]
#[ignore]
async fn test_integration_get_stations() {
    let api = ComboiosApi::new();
    let result = api.get_stations("Lisboa").await;

    if let Ok(stations) = result {
        assert!(!stations.response.is_empty(), "Should find Lisboa stations");
        println!("Found {} stations", stations.response.len());
        for station in stations.response.iter().take(3) {
            println!("  - {} (ID: {})", station.designation, station.code);
        }
    } else {
        eprintln!(
            "Integration test failed (expected with network issues): {}",
            result.unwrap_err()
        );
    }
}

#[tokio::test]
#[ignore]
async fn test_integration_chain_calls() {
    let api = ComboiosApi::new();

    let stations = match api.get_stations("Lisboa").await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to get stations: {}", e);
            return;
        }
    };

    let station = match stations.response.first() {
        Some(s) => s,
        None => {
            eprintln!("No stations found");
            return;
        }
    };

    // Get departures for next 12 hours
    let start = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
    let end = (chrono::Local::now() + chrono::Duration::hours(12))
        .format("%Y-%m-%d %H:%M")
        .to_string();

    match api.get_station_timetable(&station.code, &start, &end).await {
        Ok(boards) => {
            let total_trains: usize = boards.iter().map(|b| b.trains.len()).sum();
            println!(
                "Found {} train entries for station {} over {} board(s)",
                total_trains,
                station.designation,
                boards.len()
            );

            // Show first few trains
            for board in boards.iter().take(1) {
                println!("\nBoard type: {}", board.station_name);
                for (i, train) in board.trains.iter().take(3).enumerate() {
                    print!(
                        "  {}. Train {} from {} to {} at {}",
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
                }
            }
        }
        Err(e) => {
            eprintln!("Timetable request failed: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_integration_get_station_board_now() {
    let api = ComboiosApi::new();

    // Get Lisboa-Oriente station
    let stations = match api.get_stations("Lisboa-Oriente").await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to get stations: {}", e);
            return;
        }
    };

    let station = match stations.response.first() {
        Some(s) => s,
        None => {
            eprintln!("No stations found");
            return;
        }
    };

    // Use convenience method to get current board
    match api.get_station_board_now(&station.code).await {
        Ok(boards) => {
            let total_trains: usize = boards.iter().map(|b| b.trains.len()).sum();
            println!(
                "Found {} upcoming trains for {} (next 12 hours)",
                total_trains, station.designation
            );
        }
        Err(e) => {
            eprintln!("Board request failed: {}", e);
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_integration_search_porto() {
    let api = ComboiosApi::new();
    let result = api.get_stations("Porto").await;

    match result {
        Ok(stations) => {
            assert!(!stations.response.is_empty(), "Should find Porto stations");
            println!("Found {} Porto stations", stations.response.len());
        }
        Err(e) => {
            eprintln!("Porto search failed: {}", e);
        }
    }
}
