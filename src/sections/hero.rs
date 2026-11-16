use dioxus::prelude::*;

use crate::components::Corners;
use crate::content::{CLOCK_LABEL, HERO};
use crate::grid::GridCanvas;

const AVATAR: Asset = asset!("/assets/avatar.jpeg");

#[component]
pub fn Hero() -> Element {
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
                BottomRow {}
            }

            div { class: "relative pl-6", aria_hidden: "true",
                div { class: "k-wordmark", "{HERO.wordmark}" }
            }
        }
    }
}

#[component]
fn TopRow() -> Element {
    rsx! {
        div { class: "mb-14 flex items-start justify-between gap-6",
            div { class: "flex items-center gap-4",
                span { class: "k-idx", "/ 00" }
                span { class: "h-px w-30 bg-fg" }
                span { class: "k-label", "индекс" }
            }
            p { class: "k-label m-0 text-right leading-[1.7] tablet:hidden",
                "сетка"
                br {}
                span { class: "text-accent-ink", "пиздит" }
            }
        }
    }
}

#[component]
fn Intro() -> Element {
    rsx! {
        div { class: "flex flex-col gap-8 pb-10",
            p { class: "k-label k-rise m-0 text-fg-soft", "{HERO.role}" }

            h1 {
                class: "k-rise m-0 font-serif text-[148px] leading-[0.9] font-normal tracking-[-0.045em] text-fg tablet:text-[72px]",
                style: "--d: 80ms",
                "{HERO.first_name}"
                br {}
                em { class: "font-light text-accent-ink", "{HERO.last_name}" }
                span { class: "k-blink ml-[0.06em] inline-block h-[0.72em] w-[0.42em] bg-accent align-[-0.02em]" }
            }

            p {
                class: "k-rise m-0 max-w-[520px] text-[21px] leading-normal text-muted",
                style: "--d: 160ms",
                "{HERO.intro}"
            }

            div { class: "k-rise flex flex-wrap gap-3", style: "--d: 240ms",
                a { class: "k-btn k-btn-accent", href: "#contact",
                    "{HERO.primary_action}"
                    ArrowIcon { d: "M5 12h14M13 6l6 6-6 6" }
                }
                a { class: "k-btn", href: "#projects",
                    "{HERO.secondary_action}"
                    ArrowIcon { d: "M7 17 17 7M8 7h9v9" }
                }
            }
        }
    }
}

#[component]
fn Photo() -> Element {
    let mut photo_failed = use_signal(|| false);

    rsx! {
        div { class: "flex justify-end pb-10",
            figure { class: "relative m-0 w-full max-w-[420px] p-[18px]",
                Corners {}

                div { class: "mb-2.5 flex justify-between",
                    span { class: "k-label text-[11px]", "FURRY_PORN.JPEG" }
                    span { class: "k-label flex items-center gap-1.5 text-[11px] text-accent-ink",
                        span { class: "size-1.5 animate-pulse rounded-full bg-current" }
                        "REC"
                    }
                }

                div { class: "relative aspect-square overflow-hidden bg-paper shadow-stroke",
                    if photo_failed() {
                        div { class: "k-hatch-soft flex size-full items-center justify-center",
                            span { class: "k-label bg-bg px-2.5 py-1.5", "ИСПУГАЛСЯ?" }
                        }
                    } else {
                        img {
                            class: "block size-full object-cover transition-[filter] duration-300 dark:invert",
                            src: AVATAR,
                            alt: "аватар",
                            onerror: move |_| photo_failed.set(true),
                        }
                    }

                    span { class: "absolute top-[22%] right-[26%] size-[22px] bg-accent" }
                    svg {
                        class: "absolute bottom-6 left-6 text-fg",
                        width: "56",
                        height: "56",
                        view_box: "0 0 56 56",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.2",
                        circle { cx: "28", cy: "28", r: "14" }
                        path { d: "M28 4v14M28 38v14M4 28h14M38 28h14" }
                    }
                }

                figcaption { class: "mt-2.5 flex justify-between gap-3",
                    span { class: "k-label text-[11px]", "{HERO.coordinates}" }
                    span { class: "k-label text-[11px]", "{HERO.city} / {CLOCK_LABEL}" }
                }
            }
        }
    }
}

#[component]
fn BottomRow() -> Element {
    rsx! {
        div { class: "flex items-center justify-between gap-5 border-t border-rule py-[18px]",
            div { class: "k-label flex items-center gap-3.5 text-fg",
                "ПОРЯДОК"
                svg {
                    width: "120",
                    height: "10",
                    view_box: "0 0 120 10",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.2",
                    path { d: "M0 5h118M112 1l5 4-5 4" }
                }
                span { class: "text-accent-ink", "ХАОС" }
            }
            span { class: "k-label", "/00" }
        }
    }
}

#[component]
fn ArrowIcon(d: &'static str) -> Element {
    rsx! {
        svg {
            width: "15",
            height: "15",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.8",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            path { d }
        }
    }
}
