use crate::api::get_station_trains;
use crate::domain::{StationBoard, TrainEntry};
use dioxus::prelude::*;

#[component]
pub fn StationScreen(station_id: String) -> Element {
    let boards = use_signal(Vec::<StationBoard>::new);
    let mut loading = use_signal(|| true);

    // Fetch trains when station_id changes
    use_effect(move || {
        let mut boards = boards.clone();
        let station_id = station_id.clone();
        spawn(async move {
            loading.set(true);
            if let Ok(results) = get_station_trains(&station_id).await {
                boards.set(results);
            }
            loading.set(false);
        });
    });

    let boards_vec = boards().clone();
    // Flatten all trains from all boards
    let all_trains: Vec<(usize, usize, TrainEntry)> = boards_vec
        .iter()
        .enumerate()
        .flat_map(|(board_idx, board)| {
            board
                .trains
                .iter()
                .enumerate()
                .map(move |(train_idx, train)| (board_idx, train_idx, train.clone()))
        })
        .collect();

    rsx! {
        div { class: "flex flex-col items-center min-h-screen bg-base-200",
            div { class: "w-full max-w-2xl p-8 bg-base-100 rounded-lg shadow-lg mt-8",
                h2 { class: "text-2xl font-bold mb-4", "Upcoming Trains" }
                if loading() {
                    div { class: "loading loading-spinner loading-lg mx-auto" }
                } else if all_trains.is_empty() {
                    div { class: "text-center text-gray-500", "No trains found." }
                } else {
                    table { class: "table w-full",
                        thead {
                            tr {
                                th { "Service" }
                                th { "Origin" }
                                th { "Destination" }
                                th { "Time" }
                                th { "Train" }
                                th { "Delay" }
                                th { "Status" }
                            }
                        }
                        tbody {
                            for (_board_idx, _train_idx, train) in all_trains.iter() {
                                tr { class: "hover:bg-base-200",
                                    td { "{train.service_type}" }
                                    td { "{train.origin_station_name}" }
                                    td { "{train.destination_station_name}" }
                                    td { "{train.display_time()}" }
                                    td { "{train.train_number}" }
                                    td {
                                        if let Some(delay) = train.delay_minutes() {
                                            if delay > 0 {
                                                span { class: "text-error font-bold", "+{delay} min" }
                                            } else {
                                                span { class: "text-success", "On time" }
                                            }
                                        } else {
                                            span { class: "text-success", "On time" }
                                        }
                                    }
                                    td {
                                        if train.has_passed {
                                            span { class: "text-gray-500", "Departed" }
                                        } else {
                                            span { class: "text-success", "Scheduled" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
