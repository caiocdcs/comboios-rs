use crate::Route;
use dioxus::prelude::*;

/// The Navbar component that will be rendered on all pages of our app since every page is under the layout.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div {
            class: "navbar bg-base-100 shadow-sm",
            div {
                class: "navbar-start",
                div {
                    class: "dropdown",
                    div {
                        tabindex: "0",
                        role: "button",
                        class: "btn btn-ghost btn-circle",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            class: "h-5 w-5",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                d: "M4 6h16M4 12h16M4 18h7"
                            }
                        }
                    }
                    ul {
                        tabindex: "0",
                        class: "menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow",
                        li {
                            Link {
                                class: "btn btn-ghost text-xl",
                                to: Route::SearchScreen {},
                                "Search"
                            }
                        }
                        li {
                            Link {
                                class: "btn btn-ghost text-xl",
                                to: Route::SearchScreen {},
                                "About"
                            }
                        }
                    }
                }
            }
            div {
                class: "navbar-center",
                h1 {
                    "CP Viewer"
                }
            }
        }

        Outlet::<Route> {}
    }
}
