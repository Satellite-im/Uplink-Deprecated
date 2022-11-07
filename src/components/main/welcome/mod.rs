use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Welcome(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            id: "welcome",
            img {
                src: "extra/assets/img/uplink_muted.svg"
            }
        }
    })
}
