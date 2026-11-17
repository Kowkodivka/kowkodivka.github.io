use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdArrowRight, LdArrowUpRight, LdAsterisk, LdCrosshair};

use crate::components::Corners;
use crate::grid::GridCanvas;
use crate::i18n::use_texts;

const AVATAR: Asset = asset!("/assets/avatar.jpeg");

#[component]
pub fn Hero() -> Element {
    let hero = &use_texts().hero;
    let mut pointer = use_signal(|| None::<(f64, f64)>);

    rsx! {
        section {
            class: "relative overflow-hidden border-b border-rule",
            onmousemove: move |event| {
                let point = event.client_coordinates();
                pointer.set(Some((point.x, point.y)));
            },
            onmouseleave: move |_| pointer.set(None),

            GridCanvas { pointer }

            div { class: "k-wrap relative z-10 pt-14",
                TopRow {}
                div { class: "grid grid-cols-[1.25fr_1fr] items-end gap-10 tablet:grid-cols-1",
                    Intro {}
                    Photo {}
                }
            }

            div { class: "relative pl-6", aria_hidden: "true",
                div { class: "k-wordmark", "{hero.wordmark}" }
            }
        }
    }
}

#[component]
fn TopRow() -> Element {
    let hero = &use_texts().hero;

    rsx! {
        div { class: "mb-14 flex items-start justify-between gap-6",
            div { class: "flex items-center gap-4",
                span { class: "k-idx", "/ 00" }
                span { class: "k-label", "{hero.index_label}" }
            }
        }
    }
}

#[component]
fn Intro() -> Element {
    let hero = &use_texts().hero;

    rsx! {
        div { class: "flex flex-col gap-8 pb-10",
            p { class: "k-label k-rise m-0 flex flex-wrap items-center gap-x-2.5 gap-y-1 text-fg-soft",
                for (i , role) in hero.roles.iter().enumerate() {
                    if i > 0 {
                        Icon {
                            icon: LdAsterisk,
                            width: 11,
                            height: 11,
                            class: "text-accent-ink",
                        }
                    }
                    span { "{role}" }
                }
            }

            h1 {
                class: "k-rise m-0 font-serif text-[148px] leading-[0.9] font-normal tracking-[-0.045em] text-fg tablet:text-[72px]",
                style: "--d: 80ms",
                "{hero.first_name}"
                br {}
                em { class: "font-light text-accent-ink", "{hero.last_name}" }
                span { class: "k-blink ml-[0.06em] inline-block h-[0.72em] w-[0.42em] bg-accent align-[-0.02em]" }
            }

            p {
                class: "k-rise m-0 max-w-[520px] text-[21px] leading-normal text-muted",
                style: "--d: 160ms",
                "{hero.intro}"
            }

            div { class: "k-rise flex flex-wrap gap-3", style: "--d: 240ms",
                a { class: "k-btn k-btn-accent", href: "#contact",
                    "{hero.primary_action}"
                    Icon { icon: LdArrowRight, width: 15, height: 15 }
                }
                a { class: "k-btn", href: "#projects",
                    "{hero.secondary_action}"
                    Icon { icon: LdArrowUpRight, width: 15, height: 15 }
                }
            }
        }
    }
}

#[component]
fn Photo() -> Element {
    let texts = use_texts();
    let hero = &texts.hero;

    rsx! {
        div { class: "flex justify-end pb-10",
            figure { class: "relative m-0 w-full max-w-[420px] p-[18px]",
                Corners {}

                div { class: "mb-2.5 flex justify-between",
                    span { class: "k-label text-[11px]", "AVATAR.JPEG" }
                    span { class: "k-label flex items-center gap-1.5 text-[11px] text-accent-ink",
                        span { class: "size-1.5 animate-pulse rounded-full bg-current" }
                        "REC"
                    }
                }

                div { class: "relative aspect-square overflow-hidden bg-paper shadow-stroke",
                    img {
                        class: "block size-full object-cover transition-[filter] duration-300 dark:invert",
                        src: AVATAR,
                        alt: hero.photo_alt,
                    }
                    span { class: "absolute top-[22%] right-[26%] size-[22px] bg-accent" }
                    Icon {
                        icon: LdCrosshair,
                        width: 56,
                        height: 56,
                        class: "absolute bottom-6 left-6 stroke-[0.6] text-fg",
                    }
                }

                figcaption { class: "mt-2.5 flex justify-between gap-3",
                    span { class: "k-label text-[11px]", "{hero.coordinates}" }
                    span { class: "k-label text-[11px]", "{hero.city} / {texts.header.clock_label}" }
                }
            }
        }
    }
}
