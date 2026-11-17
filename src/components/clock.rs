use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

use crate::content::CLOCK_OFFSET_HOURS;
use crate::i18n::use_texts;

#[component]
pub fn Clock() -> Element {
    let texts = use_texts();
    let mut time = use_signal(current_time);

    use_future(move || async move {
        loop {
            TimeoutFuture::new(1000).await;
            time.set(current_time());
        }
    });

    rsx! {
        span { class: "k-label tabular-nums text-fg-soft tablet:hidden",
            "{texts.header.clock_label} {time}"
        }
    }
}

fn current_time() -> String {
    let ms = js_sys::Date::now() as i64 + CLOCK_OFFSET_HOURS * 3_600_000;
    let seconds_today = (ms / 1000).rem_euclid(24 * 60 * 60);

    let hours = seconds_today / 3600;
    let minutes = seconds_today / 60 % 60;
    let seconds = seconds_today % 60;

    format!("{hours:02}:{minutes:02}:{seconds:02}")
}
