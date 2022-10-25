use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::components::ui_kit::{button::Button, icon_button::IconButton};

#[derive(Props, PartialEq)]
pub struct Props {
    account: crate::Account,
}

#[allow(non_snake_case)]
pub fn Toolbar(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            id: "toolbar",
            div {
                id: "controls",
                IconButton {
                    icon: Shape::Archive,
                    state: crate::components::ui_kit::icon_button::State::Secondary,
                    on_pressed: move |_| {}
                },
                Button {
                    icon: Shape::FolderAdd,
                    text: String::from("New Folder"),
                    state: crate::components::ui_kit::button::State::Secondary,
                    on_pressed: move |_| {}
                },
                IconButton {
                    icon: Shape::Upload,
                    on_pressed: move |_| {}
                }
            },
            div {
                id: "close",
                IconButton {
                    on_pressed: move |_| {
                        use_router(&cx).push_route("/main", None, None);
                    },
                    icon: Shape::X
                }
            }
        }
    })
}
