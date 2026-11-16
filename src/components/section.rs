use dioxus::prelude::*;

use crate::content::SectionInfo;

#[component]
pub fn Section(info: SectionInfo, #[props(default)] class: String, children: Element) -> Element {
    rsx! {
        section { id: info.id, class: "py-28 tablet:py-20 {class}",
            div { class: "k-wrap flex flex-col gap-10",
                SectionHeading { info }
                {children}
            }
        }
    }
}

#[component]
fn SectionHeading(info: SectionInfo) -> Element {
    rsx! {
        div { class: "flex flex-col gap-5",
            div { class: "flex items-center gap-4",
                span { class: "k-idx", "/ {info.number}" }
                span { class: "k-label", "{info.label}" }
            }
            h2 { class: "k-h2",
                "{info.title}"
                em { "{info.accent}" }
                "{info.title_end}"
            }
        }
    }
}
