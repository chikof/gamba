use crate::Route;
use daisy_rsx::*;
use dioxus::prelude::*;

// const NAVBAR_CSS: Asset = asset!("/assets/tailwind.css");

#[component]
pub fn Navbar() -> Element {
    let mut session: Signal<Option<String>> = use_signal(|| None);

    rsx! {
        // document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        //
        nav { class: "bg-primary text-primary-foreground",
            div {
                Link { to: Route::Home {}, class: "text-lg font-bold", "Bookmaker Tracker" }

                div { class: "flex items-center space-x-4",
                    span { "Welcome, Chiko" }
                    Button {
                        class: "btn-circle mr-2 p-1",
                        button_scheme: ButtonScheme::Primary,
                        "Logout"
                    }
                }
            }
        }

        Outlet::<Route> {}
    }
}
