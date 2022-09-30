use dioxus::prelude::*;

use crate::utils::config::load_or_default;

#[derive(Props)]
pub struct Props<'a> {
    show: bool,
    on_hide: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Settings<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let config = load_or_default();

    cx.render(rsx! {
        div {
            id: "settings",
            div {
                id: "content",
                "settings"
            }
        }
    })
}
