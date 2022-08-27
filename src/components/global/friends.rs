use dioxus::{prelude::*, events::FormEvent};
use dioxus_heroicons::{Icon, outline::Shape};
use sir::global_css;
use warp::tesseract::Tesseract;

use crate::components::ui_kit::{popup::Popup, input::Input, icon_input::IconInput, icon_button::IconButton};

#[derive(PartialEq, Props)]
pub struct Props {
    tesseract: Tesseract,
    icon: Shape,
    title: String,
}

#[allow(non_snake_case)]
pub fn Friends(cx: Scope<Props>) -> Element {
    global_css! {"
        .friends {
            display: inline-flex;
            flex-direction: column;

            .add {
                display: inline-flex;
                flex-direction: row;

                .icon-input {
                    width: 100%;
                    margin-right: 1rem;
                }
            }
        }
    "}

    cx.render(rsx!{
        Popup {
            tesseract: cx.props.tesseract.clone(),
            children: cx.render(rsx!(
                div {
                    class: "friends",
                    div {
                        class: "title",
                        Icon {
                            icon: cx.props.icon,
                            size: 20,
                        },
                        "{cx.props.title}"
                    },
                    label {
                        "Add Someone"
                    }
                    div {
                        class: "add",
                        IconInput {
                            placeholder: "Warp#a3fdc6..".to_string(),
                            icon: Shape::UserAdd,
                            oninput: move | _ | {},
                        }
                        IconButton {
                            icon: Shape::Plus,
                            onclick:  move | _ | {},
                        }
                    },
                    label {
                        "Your Friends"
                    }
                }
            ))
        },
    })
}