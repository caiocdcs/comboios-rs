use crate::api::get_station_trains;
use crate::domain::Timetable;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn StationScreen(station_id: String) -> Element {
    let timetable = use_signal(Vec::<Timetable>::new);
    let mut loading = use_signal(|| true);
    let nav = use_navigator();

    // Fetch trains when station_id changes
    use_effect(move || {
        let mut timetable = timetable.clone();
        let station_id = station_id.clone();
        spawn(async move {
            loading.set(true);
            if let Ok(results) = get_station_trains(&station_id).await {
                timetable.set(results);
            }
            loading.set(false);
        });
    });

    let timetable_vec = timetable().clone();

    rsx! {
        div { class: "flex flex-col items-center min-h-screen bg-base-200",
            div { class: "w-full max-w-2xl p-8 bg-base-100 rounded-lg shadow-lg mt-8",
                h2 { class: "text-2xl font-bold mb-4", "Upcoming Trains" }
                if loading() {
                    div { class: "loading loading-spinner loading-lg mx-auto" }
                } else if timetable_vec.is_empty() {
                    div { class: "text-center text-gray-500", "No trains found." }
                } else {
                    table { class: "table w-full",
                        thead {
                            tr {
                                th { "Origin" }
                                th { "Destination" }
                                th { "Arrival" }
                                th { "Departure" }
                                th { "ETA" }
                                th { "ETD" }
                                th { "Platform" }
                                th { "Occupancy" }
                                th { "Delay" }
                                th { "Train Number" }
                            }
                        }
                        tbody {
                            for stopover in timetable.iter() {
                                tr { class: "hover:bg-base-200 cursor-pointer",
                                    onclick: {
                                        let nav = nav.clone();
                                        let id = stopover.train_number.to_string();
                                        move |_| {
                                            nav.push(Route::TrainScreen { train_id: id.clone() });
                                        }
                                    },
                                    td { "{stopover.train_origin.name.clone()}" }
                                    td { "{stopover.train_destination.name.clone()}" }
                                    td { "{stopover.arrival_time.clone().unwrap_or_default()}" }
                                    td { "{stopover.departure_time.clone().unwrap_or_default()}" }
                                    td { "{stopover.eta.clone().unwrap_or_default()}" }
                                    td { "{stopover.etd.clone().unwrap_or_default()}" }
                                    td { "{stopover.platform.clone().unwrap_or_default()}" }
                                    td { "{stopover.occupancy.clone().unwrap_or_default()}" }
                                    td { "{stopover.delay.clone().unwrap_or_default()}" }
                                    td { "{stopover.train_number.clone()}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
