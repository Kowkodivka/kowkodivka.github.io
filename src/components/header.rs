use dioxus::prelude::*;

use crate::content::{NICK, SECTIONS};

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "sticky top-0 z-30 border-b border-rule bg-bg/90 backdrop-blur-sm",
            div { class: "k-wrap flex h-16 items-center justify-between gap-6",
                Logo {}
                Nav {}
                div { class: "flex items-center gap-3.5" }
            }
        }
    }
}

#[component]
pub fn Logo() -> Element {
    rsx! {
        a {
            class: "flex items-center gap-2.5 font-mono text-sm font-semibold tracking-tight text-fg no-underline",
            href: "#top",
            span { class: "inline-block size-3.5 bg-accent" }
            "{NICK}"
        }
    }
}

#[component]
pub fn Nav() -> Element {
    rsx! {
        nav { class: "flex gap-5 tablet:hidden", aria_label: "разделы",
            for section in SECTIONS {
                a { class: "k-nav-link", href: "#{section.id}",
                    span { "{section.number}" }
                    "{section.label}"
                }
            }
        }
    }
}
