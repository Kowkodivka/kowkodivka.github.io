use dioxus::prelude::*;

use crate::i18n::{Lang, set_lang, use_lang, use_texts};

#[component]
pub fn LangToggle() -> Element {
    let lang = use_lang();
    let texts = use_texts();

    rsx! {
        button {
            class: "k-icon-btn w-auto gap-1.5 px-2.5 font-mono text-[12px] font-medium",
            r#type: "button",
            aria_label: texts.header.lang_button,
            onclick: move |_| set_lang(lang, lang().other()),
            span { class: if lang() == Lang::Ru { "text-fg" } else { "text-faint" }, "RU" }
            span { class: "text-faint", "/" }
            span { class: if lang() == Lang::En { "text-fg" } else { "text-faint" }, "EN" }
        }
    }
}
