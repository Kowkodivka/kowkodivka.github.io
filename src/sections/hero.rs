use dioxus::prelude::*;

use crate::grid::GridCanvas;

#[component]
pub fn Hero() -> Element {
    let mut pointer = use_signal(|| None::<(f64, f64)>);

    rsx! {
        section {
            class: "relative h-[calc(100vh-4rem)] overflow-hidden border-b border-rule",
            onmousemove: move |event| {
                let point = event.client_coordinates();
                pointer.set(Some((point.x, point.y)));
            },
            onmouseleave: move |_| pointer.set(None),

            GridCanvas { pointer }
        }
    }
}
