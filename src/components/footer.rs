use dioxus::prelude::*;

use crate::content::NICK;

#[component]
pub fn Footer() -> Element {
    let year = use_hook(|| js_sys::Date::new_0().get_full_year());

    rsx! {
        footer { class: "border-t border-fg",
            div { class: "k-wrap flex items-center justify-between gap-6 py-7 tablet:flex-col tablet:items-start tablet:gap-2",
                span { class: "k-label text-[11px]", "(C) {year} {NICK}" }
                a {
                    class: "k-label text-[11px] text-fg no-underline hover:text-accent-ink",
                    href: "#top",
                    "наверх"
                }
            }
        }
    }
}
