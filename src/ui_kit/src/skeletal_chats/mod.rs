use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn SkeletalChats(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "skeletal-stack",
            div {
                class: "skele-chat",
                div { class: "circle" },
                div {
                    class: "right-side",
                    div { class: "rect" },
                    div { class: "rect" }
                }
            },
            div {
                class: "skele-chat",
                div { class: "circle" },
                div {
                    class: "right-side",
                    div { class: "rect" },
                    div { class: "rect" }
                }
            },
            div {
                class: "skele-chat",
                div { class: "circle" },
                div {
                    class: "right-side",
                    div { class: "rect" },
                    div { class: "rect" }
                }
            },
        }
    })
}
