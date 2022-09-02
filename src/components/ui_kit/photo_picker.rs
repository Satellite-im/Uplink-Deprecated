use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};

use crate::components::ui_kit::icon_button::IconButton;

#[derive(PartialEq, Props)]
pub struct Props {
    text: Option<String>,
}

#[allow(non_snake_case)]
pub fn PhotoPicker(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            class: "photo-picker",
            div {
                class: "display",
                Icon {
                    icon: Shape::User,
                    size: 30,
                }
            }
            IconButton {
                icon: Shape::Plus,
                on_pressed: move |_| {},
            }
        }
    })
}
