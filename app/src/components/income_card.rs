use daisy_rsx::*;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IncomeCardProps {
    pub label: String,
    pub income: f64,
}

#[component]
pub fn IncomeCard(props: IncomeCardProps) -> Element {
    let income_str = format!("${:.2}", props.income);

    let color = if props.income > 0.0 {
        "text-green-600"
    } else {
        "text-red-600"
    };

    rsx! {
        Card {
            class: "h-24 card",
            CardHeader {
                class: "p-3 text-lg font-bold",
                title: "{props.label}",
            }
            CardBody {
                class: "p-3 pt-0",
                p {
                    class: "text-2xl font-bold {color}",
                    "{income_str}"
                }
            }
        }
    }
}
