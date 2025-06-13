use crate::api::search_station;
use crate::domain::Station;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn SearchScreen() -> Element {
    let mut search = use_signal(String::new);
    let stations = use_signal(Vec::<Station>::new);
    let loading = use_signal(|| false);
    let nav = use_navigator();

    let do_search = move || {
        let search = search();
        let mut stations = stations.clone();
        let mut loading = loading.clone();
        async move {
            loading.set(true);
            match search_station(&search).await {
                Ok(results) => {
                    stations.set(results);
                }
                Err(e) => {
                    log::error!("Error fetching stations: {:?}", e);
                    dbg!(&e); // Print the error if fetch fails
                }
            }
            loading.set(false);
        }
    };

    rsx! {
        div { class: "flex flex-col items-center bg-base-200 min-h-screen",
            div { class: "w-full max-w-md p-8 bg-base-100 rounded-lg shadow-lg mt-12 space-y-6",
                h2 { class: "text-3xl font-bold", "Find a Station" }
                input {
                    class: "input input-bordered mb-3",
                    placeholder: "Enter station name...",
                    value: "{search()}",
                    oninput: move |e| search.set(e.value().to_string()),
                    onkeydown: {
                        let do_search = do_search.clone();
                        move |e: KeyboardEvent| {
                            if e.key() == Key::Enter {
                                spawn(do_search());
                            }
                        }
                    }
                }
                button {
                    class: "btn btn-xs sm:btn-sm md:btn-md lg:btn-lg xl:btn-xl",
                    onclick: move |_| {
                        spawn(do_search());
                    },
                    "Search"
                }
                if loading() {
                    div { class: "loading loading-spinner loading-lg mx-auto my-8" }
                } else {
                    ul { class: "menu bg-base-100 rounded-box space-y-2",
                        {
                            stations.with(|vec| {
                                vec.iter().map(|station| {
                                    rsx! {
                                        li {
                                            a {
                                                class: "hover:bg-primary hover:text-primary-content transition px-4 py-3",
                                                onclick: {
                                                    let nav = nav.clone();
                                                    let id = station.id.clone();
                                                    move |_| {
                                                        nav.push(Route::StationScreen { station_id: id.clone() });
                                                    }
                                                },
                                                "{station.name}"
                                            }
                                        }
                                    }
                                }).collect::<Vec<_>>().into_iter()
                            })
                        }
                    }
                }
            }
        }
    }
}
