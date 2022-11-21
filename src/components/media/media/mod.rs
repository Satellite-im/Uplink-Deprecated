use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Media(cx: Scope) -> Element {
    log::debug!("rendering Media");

    cx.render(rsx! {
        div {
            class: "media",
            div {
                class: "content"
            }
        }
    })
}
