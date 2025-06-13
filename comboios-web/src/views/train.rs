use crate::{api::get_train_details, domain::TrainDetails};
use dioxus::prelude::*;

#[component]
pub fn TrainScreen(train_id: String) -> Element {
    let details = use_signal(|| None as Option<TrainDetails>);
    let loading = use_signal(|| true);

    use_effect(move || {
        let mut details = details.clone();
        let mut loading = loading.clone();
        let train_id = train_id.clone();
        spawn(async move {
            loading.set(true);
            match get_train_details(&train_id).await {
                Ok(result) => {
                    details.set(Some(result));
                }
                Err(e) => {
                    log::error!("Error fetching train details: {:?}", e);
                }
            }
            loading.set(false);
        });
    });

    let details_val = details();

    rsx! {
        div { class: "flex flex-col items-center min-h-screen bg-base-200",
            div { class: "w-full max-w-3xl p-8 bg-base-100 rounded-lg shadow-lg mt-8",
                if loading() {
                    div { class: "loading loading-spinner loading-lg mx-auto" }
                } else if let Some(details) = details_val {
                    h2 { class: "text-2xl font-bold mb-4", "Train #{details.id}" }
                    div { class: "mb-2", "Status: ", span { class: "badge badge-info", "{details.status.clone().unwrap_or_default()}" } }
                    div { class: "mb-2", "Delay: ", "{details.delay.map(|d| d.to_string()).unwrap_or_default()} min" }
                    div { class: "mb-2", "Latitude: ", "{details.latitude.clone().unwrap_or_default()}" }
                    div { class: "mb-2", "Longitude: ", "{details.longitude.clone().unwrap_or_default()}" }
                    div { class: "mb-2", "Occupancy: ", "{details.occupancy.clone().unwrap_or_default()}" }
                    h3 { class: "text-lg font-semibold mt-4 mb-2", "Route" }
                    table { class: "table w-full",
                        thead {
                            tr {
                                th { "Station" }
                                th { "Arrival" }
                                th { "Departure" }
                                th { "Platform" }
                                th { "ETA" }
                                th { "ETD" }
                                th { "Delay" }
                                th { "Latitude" }
                                th { "Longitude" }
                            }
                        }
                        tbody {
                            if let Some(stops) = &details.stops {
                                for stop in stops.iter() {
                                    tr {
                                        td { "{stop.station.name.clone()}" }
                                        td { "{stop.arrival_time.clone().unwrap_or_default()}" }
                                        td { "{stop.departure_time.clone().unwrap_or_default()}" }
                                        td { "{stop.platform.clone().unwrap_or_default()}" }
                                        td { "{stop.eta.clone().unwrap_or_default()}" }
                                        td { "{stop.etd.clone().unwrap_or_default()}" }
                                        td { "{stop.delay.map(|d| d.to_string()).unwrap_or_default()}" }
                                        td { "{stop.latitude.clone()}" }
                                        td { "{stop.longitude.clone()}" }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div { class: "text-gray-500 text-center mt-12", "No train details found." }
                }
            }
        }
    }
}
