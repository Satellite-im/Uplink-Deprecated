use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Badge(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "badge-renderer",

        }
    })
}
