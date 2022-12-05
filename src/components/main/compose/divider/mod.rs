use dioxus::prelude::*;

#[derive(Props, PartialEq)]
pub struct Props {}

#[allow(non_snake_case)]
pub fn Divider<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "msg-divider",
            div {
                class: "msg-divider-label",
                "New messages"
            }
            div {
                class: "msg-divider-line",
            }
        }
    })
}
