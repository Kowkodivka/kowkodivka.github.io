use dioxus::prelude::*;

use crate::i18n::use_texts;

#[component]
pub fn Footer() -> Element {
    let texts = use_texts();
    let year = use_hook(|| js_sys::Date::new_0().get_full_year());

    rsx! {
        footer { class: "border-t border-fg",
            div { class: "k-wrap flex items-center justify-between gap-6 py-7 tablet:flex-col tablet:items-start tablet:gap-2",
                span { class: "k-label text-[11px]", "(C) {year} {texts.nick}" }
                a {
                    class: "k-label text-[11px] text-fg no-underline hover:text-accent-ink",
                    href: "#top",
                    "{texts.footer.to_top}"
                }
            }
        }
    }
}
