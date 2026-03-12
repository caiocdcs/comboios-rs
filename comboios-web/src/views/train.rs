use crate::api::get_train_details;
use dioxus::prelude::*;

#[component]
pub fn TrainScreen(train_id: String) -> Element {
    let message = use_signal(|| None as Option<String>);
    let loading = use_signal(|| true);

    use_effect(move || {
        let mut message = message.clone();
        let mut loading = loading.clone();
        let train_id = train_id.clone();
        spawn(async move {
            loading.set(true);
            match get_train_details(&train_id).await {
                Ok(result) => {
                    message.set(Some(result));
                }
                Err(e) => {
                    log::error!("Error fetching train details: {:?}", e);
                    message.set(Some("Failed to fetch train details".to_string()));
                }
            }
            loading.set(false);
        });
    });

    let message_val = message();

    rsx! {
        div { class: "flex flex-col items-center min-h-screen bg-base-200",
            div { class: "w-full max-w-3xl p-8 bg-base-100 rounded-lg shadow-lg mt-8",
                if loading() {
                    div { class: "loading loading-spinner loading-lg mx-auto" }
                } else if let Some(msg) = message_val {
                    div { class: "alert alert-warning",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "stroke-current shrink-0 h-6 w-6",
                            fill: "none",
                            view_box: "0 0 24 24",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z",
                            }
                        }
                        span { "{msg}" }
                    }
                    div { class: "mt-4 text-gray-600",
                        "The train details feature has been deprecated because the Comboios de Portugal API is no longer available. Please use the station timetable view to see current train information."
                    }
                } else {
                    div { class: "text-gray-500 text-center mt-12", "No message available." }
                }
            }
        }
    }
}
