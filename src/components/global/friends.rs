use dioxus::prelude::*;
use dioxus_heroicons::{Icon, outline::Shape};
use sir::global_css;
use warp::tesseract::Tesseract;

use crate::components::ui_kit::{popup::Popup, icon_input::IconInput, icon_button::IconButton};

#[derive(Props)]
pub struct Props<'a> {
    tesseract: Tesseract,
    icon: Shape,
    title: String,
    handle_close: EventHandler<'a>,
}

#[allow(non_snake_case)]
pub fn Friends<'a>(cx: Scope<Props<'a>>) -> Element<'a> {
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
            close: cx.props.handle_close,
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