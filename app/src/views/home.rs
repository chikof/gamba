use crate::components::{Hero, IncomeCard, TransactionTable};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        main {
            class: "flex-grow container mx-auto p-4",
            h1 {
                class: "text-2xl font-bold mb-4",
                "Bookmaker Tracker"
            }
            div {
                class: "flex flex-col md:flex-row gap-4",
                div {
                    class: "w-full md:w-1/2",
                    TransactionTable {}
                }
                div {
                    class: "w-full md:w-1/2",
                    div {
                        class: "grid grid-cols-2 gap-4 mb-4",
                        IncomeCard { label: "Monthly Income", income: 100.0 }
                        IncomeCard { label: "Total Income", income: 100.0 }
                    }
                }
            }
        }
    }
}
