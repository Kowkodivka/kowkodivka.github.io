use dioxus::prelude::*;

const STYLES: Asset = asset!("/assets/tailwind.css");
const FONTS: &str = "https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:ital,wght@0,400;0,500;0,600;1,400\
&family=Source+Serif+4:ital,opsz,wght@0,8..60,300..600;1,8..60,300..600&display=swap";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut dark = use_signal(|| false);

    let toggle_theme = move |_| {
        let next = !dark();
        dark.set(next);
        let name = if next { "dark" } else { "light" };
        document::eval(&format!(
            "document.documentElement.dataset.theme = '{name}'"
        ));
    };

    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous",
        }
        document::Stylesheet { href: FONTS }
        document::Stylesheet { href: STYLES }

        header { class: "sticky top-0 z-30 border-b border-rule bg-bg/90 backdrop-blur-sm",
            div { class: "k-wrap flex h-16 items-center justify-between gap-6",
                span { class: "flex items-center gap-2.5 font-mono text-sm font-semibold text-fg",
                    span { class: "inline-block size-3.5 bg-accent" }
                    "STYLE CHECK"
                }
                span { class: "k-label tablet:hidden", "ширина > 900px" }
                span { class: "k-label hidden tablet:inline", "ширина ≤ 900px" }
                button {
                    class: "k-btn py-2",
                    r#type: "button",
                    onclick: toggle_theme,
                    if dark() {
                        "светлая"
                    } else {
                        "тёмная"
                    }
                }
            }
        }

        main { class: "k-wrap flex flex-col gap-24 py-20",
            Typography {}
            Palette {}
            Buttons {}
            Controls {}
            Details {}
            TextAndCode {}
            Band {}
        }
    }
}

#[component]
fn Block(index: &'static str, label: &'static str, children: Element) -> Element {
    rsx! {
        section { class: "flex flex-col gap-8",
            div { class: "flex items-center gap-4",
                span { class: "k-idx", "/ {index}" }
                span { class: "k-label", "{label}" }
                span { class: "h-px flex-1 bg-rule" }
            }
            {children}
        }
    }
}

#[component]
fn Typography() -> Element {
    rsx! {
        Block { index: "01", label: "типографика",
            h1 { class: "display k-rise",
                "Тёплая бумага, "
                em { "засечки" }
                " и моноширинный"
                span { class: "caret" }
            }
            h2 { class: "k-h2 k-rise", style: "--d: 80ms",
                "Заголовок "
                em { "секции" }
            }
            p { class: "lead max-w-[640px]",
                "Вводный абзац 20px. Текст с засечками, а всё машинное — моноширинное."
            }
            div { class: "flex flex-wrap items-center gap-6",
                span { class: "eyebrow", "eyebrow" }
                span { class: "meta", "meta · 2026-11-16 · 14:32:05" }
                span { class: "k-code", "K-CODE_001" }
                span { class: "subtext p-0", "subtext под элементом" }
            }
        }
    }
}

#[component]
fn Palette() -> Element {
    let swatches = [
        ("bg", "bg-bg"),
        ("paper", "bg-paper"),
        ("card", "bg-card"),
        ("fg", "bg-fg"),
        ("muted", "bg-muted"),
        ("faint", "bg-faint"),
        ("accent", "bg-accent"),
        ("accent-fill", "bg-accent-fill"),
        ("green", "bg-green"),
        ("blue", "bg-blue"),
        ("purple", "bg-purple"),
        ("red", "bg-red"),
    ];
    rsx! {
        Block { index: "02", label: "палитра",
            div { class: "grid grid-cols-6 gap-3 tablet:grid-cols-3",
                for (name , class) in swatches {
                    div { class: "flex flex-col gap-2",
                        div { class: "h-16 shadow-stroke {class}" }
                        span { class: "meta", "{name}" }
                    }
                }
            }
        }
    }
}

#[component]
fn Buttons() -> Element {
    rsx! {
        Block { index: "03", label: "кнопки",
            div { class: "flex flex-wrap items-center gap-3",
                a { class: "k-btn k-btn-accent", href: "#", "k-btn-accent →" }
                a { class: "k-btn", href: "#", "k-btn ↗" }
                span { class: "w-6" }
                button { class: "btn btn-accent", "btn-accent" }
                button { class: "btn", "btn" }
                button { class: "btn btn-elevated", "btn-elevated" }
                button { class: "btn btn-active", "btn-active" }
                button { class: "btn btn-ghost", "btn-ghost" }
                button { class: "btn", disabled: true, "disabled" }
            }
        }
    }
}

#[component]
fn Controls() -> Element {
    let mut tab = use_signal(|| 0usize);
    let mut on = use_signal(|| true);
    let tabs = ["всё", "код", "текст"];

    rsx! {
        Block { index: "04", label: "управление",
            div { class: "grid grid-cols-2 gap-6 tablet:grid-cols-1",
                div { class: "card",
                    span { class: "eyebrow", "switcher-big" }
                    div { class: "switcher-big",
                        for (i , name) in tabs.iter().enumerate() {
                            button {
                                class: if tab() == i { "btn btn-active" } else { "btn" },
                                onclick: move |_| tab.set(i),
                                "{name}"
                            }
                        }
                    }
                    span { class: "eyebrow", "omnibox" }
                    label { class: "omnibox",
                        input {
                            r#type: "text",
                            placeholder: "найти в заметках",
                        }
                    }
                    input { class: "input-cb", placeholder: "input-cb" }
                }
                div { class: "card",
                    span { class: "eyebrow", "toggle" }
                    button {
                        class: "btn setting-row",
                        role: "switch",
                        aria_checked: "{on}",
                        onclick: move |_| on.toggle(),
                        span { "переключатель" }
                        span { class: "toggle",
                            span { class: "toggle-knob" }
                        }
                    }
                    span { class: "eyebrow", "progress" }
                    div { class: "progress-cb",
                        span { style: "width: 68%" }
                    }
                    div { class: "skeleton h-4 w-2/3" }
                }
            }
        }
    }
}

#[component]
fn Details() -> Element {
    rsx! {
        Block { index: "05", label: "детали",
            div { class: "flex flex-wrap items-center gap-3",
                span { class: "tag", "rust" }
                span { class: "tag", "dioxus" }
                span { class: "tag tag-accent", "новое" }
                span { class: "status", "собрано" }
                span {
                    class: "status",
                    style: "--status-color: var(--color-red)",
                    "ошибка"
                }
                span { class: "meta",
                    "поиск "
                    kbd { class: "kbd", "ctrl" }
                    " "
                    kbd { class: "kbd", "K" }
                }
                span { class: "k-label", "мигает: " }
                span { class: "k-blink inline-block size-3 bg-accent" }
            }
            div { class: "rule-label", "rule-label · 05" }
            div { class: "grid grid-cols-4 gap-4 tablet:grid-cols-2",
                div { class: "relative flex aspect-square items-center justify-center p-4",
                    span { class: "k-corner k-corner-tl" }
                    span { class: "k-corner k-corner-tr" }
                    span { class: "k-corner k-corner-bl" }
                    span { class: "k-corner k-corner-br" }
                    span { class: "k-label", "k-corner" }
                }
                div { class: "k-hatch-soft flex aspect-square items-center justify-center bg-paper shadow-stroke",
                    span { class: "k-label bg-bg px-2", "k-hatch-soft" }
                }
                div { class: "k-chamfer flex aspect-square items-center justify-center bg-accent-fill",
                    span { class: "k-label text-accent-on", "k-chamfer" }
                }
                div { class: "paper-grid flex aspect-square items-center justify-center shadow-stroke",
                    span { class: "k-label", "paper-grid" }
                }
            }
            div { class: "k-hatch h-3 w-40" }
        }
    }
}

#[component]
fn TextAndCode() -> Element {
    rsx! {
        Block { index: "06", label: "текст и код",
            div { class: "grid grid-cols-[1.1fr_1fr] gap-10 tablet:grid-cols-1",
                article { class: "prose-clay",
                    h2 { "Заметка о методе" }
                    p {
                        "Текст 17px, межстрочный 1.7. Инлайн-код: "
                        code { "propagate()" }
                        ", ссылка: "
                        a { href: "#", "с глиняным подчёркиванием" }
                        "."
                    }
                    ul {
                        li { "маркер списка — тире" }
                        li { "второй пункт" }
                    }
                    blockquote {
                        "Сначала заставь работать, потом сделай красиво."
                    }
                }
                figure { class: "code-block self-start",
                    figcaption {
                        span { "src/main.rs" }
                        span { "rust" }
                    }
                    pre {
                        span { class: "ln",
                            span { class: "c",
                                "// подсветка классами k / s / c / n / f"
                            }
                        }
                        span { class: "ln",
                            span { class: "k", "fn " }
                            span { class: "f", "main" }
                            "() {{"
                        }
                        span { class: "ln",
                            "    "
                            span { class: "f", "launch" }
                            "("
                            span { class: "s", "\"app\"" }
                            ", "
                            span { class: "n", "42" }
                            ");"
                        }
                        span { class: "ln", "}}" }
                    }
                }
            }
            table { class: "table-clay",
                thead {
                    tr {
                        th { "опыт" }
                        th { "метод" }
                        th { class: "num", "значение" }
                    }
                }
                tbody {
                    tr {
                        td { "Брусок" }
                        td { "плоскость" }
                        td { class: "num", "0.312" }
                    }
                    tr {
                        td { "Гидростатика" }
                        td { "взвешивание" }
                        td { class: "num", "998.4" }
                    }
                }
            }
        }
    }
}

#[component]
fn Band() -> Element {
    rsx! {
        section { class: "k-band k-chamfer-tl",
            div { class: "k-hatch-band h-2.5 opacity-20" }
            div { class: "flex flex-col gap-6 p-14 tablet:p-8",
                div { class: "flex items-center gap-4",
                    span { class: "k-idx text-[var(--k-band-accent)]!", "/ 07" }
                    span { class: "k-label text-[var(--k-band-muted)]!", "тёмная полоса" }
                }
                h2 { class: "k-h2 text-[var(--k-band-fg)]!",
                    "Не зависит "
                    em { class: "text-[var(--k-band-accent)]!", "от темы" }
                }
                div { class: "grid grid-cols-[120px_1fr] items-baseline gap-5 border-t border-[var(--k-band-rule)] pt-5",
                    span { class: "font-mono text-5xl font-medium tracking-tighter",
                        "00"
                    }
                    span { class: "font-mono text-[13px] text-[var(--k-band-muted)]",
                        "подпись к цифре"
                    }
                }
            }
        }
    }
}
