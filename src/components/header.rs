use dioxus::prelude::*;

use crate::{
    components::{Clock, LangToggle, ThemeToggle},
    i18n::use_texts,
};

#[component]
pub fn Header() -> Element {
    rsx! {
        header { class: "sticky top-0 z-30 border-b border-rule bg-bg/90 backdrop-blur-sm",
            div { class: "k-wrap flex h-16 items-center justify-between gap-6",
                Logo {}
                Nav {}
                div { class: "flex items-center gap-3.5",
                    Clock {}
                    LangToggle {}
                    ThemeToggle {}
                }
            }
        }
    }
}

#[component]
pub fn Logo() -> Element {
    let texts = use_texts();

    rsx! {
        a {
            class: "flex items-center gap-2.5 font-mono text-sm font-semibold tracking-tight text-fg no-underline",
            href: "#top",
            span { class: "inline-block size-3.5 bg-accent" }
            "{texts.nick}"
        }
    }
}

#[component]
pub fn Nav() -> Element {
    let texts = use_texts();

    rsx! {
        nav {
            class: "flex gap-5 tablet:hidden",
            aria_label: texts.header.nav_label,
            for section in texts.sections {
                a { class: "k-nav-link", href: "#{section.id}",
                    span { "{section.number}" }
                    "{section.label}"
                }
            }
        }
    }
}
