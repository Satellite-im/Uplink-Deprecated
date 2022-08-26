use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};

use crate::components::ui_kit::icon_button::IconButton;

#[derive(PartialEq, Props)]
pub struct Props {
    text: Option<String>
}

pub fn css() -> String {"
    .photo-picker {
        width: 100px;
        height: 100px;
        background: var(--theme-secondary);
        border-radius: 50px;
        display: inline-block;
        position: relative;
    }
    .photo-picker .display {
        height: 100%;
        width: 100%;
        display: inline-flex;
        justify-content: center;
    }
    .photo-picker .display svg {
        display: inline-block;
        vertical-align: middle;
        align-self: center;
        fill: transparent;
        stroke: var(--theme-text-muted);
    }
    .photo-picker .icon-button {
        position: absolute;
        bottom: 0;
        right: 0;
        min-width: 30px;
        height: 30px;
        overflow: hidden;
        padding: 0;
    }
    .photo-picker .icon-button svg {
        padding-top: 2px;
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
                    icon: Shape::User,
                    size: 30,
                }
            }
            IconButton {
                icon: Shape::Plus,
                onclick: move |_| {},
            }
        }
    })
}