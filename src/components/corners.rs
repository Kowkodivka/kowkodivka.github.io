use dioxus::prelude::*;

#[component]
pub fn Corners() -> Element {
    rsx! {
        span { class: "k-corner k-corner-tl" }
        span { class: "k-corner k-corner-tr" }
        span { class: "k-corner k-corner-bl" }
        span { class: "k-corner k-corner-br" }
    }
}
