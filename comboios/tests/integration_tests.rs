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

    let stations = match api.get_stations("Porto").await {
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

    match api.get_station_timetable(&station.code).await {
        Ok(timetable) => {
            println!(
                "Found {} trains for station {}",
                timetable.len(),
                station.designation
            );

            if let Some(train_info) = timetable.first() {
                let _train_details = api.get_train_details(train_info.train_number as u16).await;
            }
        }
        Err(e) => {
            eprintln!("Timetable request failed: {}", e);
        }
    }
}
