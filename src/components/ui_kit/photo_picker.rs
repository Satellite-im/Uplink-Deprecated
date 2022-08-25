use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};

use crate::components::ui_kit::icon_button::IconButton;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    text: Option<String>
}

pub fn css() -> String {"
    .photo-picker {
        width: 60px;
        heigth: 60px;
        background: red;
    }
    ".to_string()}

#[allow(non_snake_case)]
pub fn PhotoPicker(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            class: "photo-picker",
            div {
                class: "display",
                Icon {
                    icon: Shape::Photograph,
                }
            }
            IconButton {
                icon: Shape::Plus,
                onclick: move |_| {},
            }
        }
    })
}