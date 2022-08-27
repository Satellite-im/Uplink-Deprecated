use dioxus::prelude::*;
use dioxus_heroicons::{Icon, outline::Shape};
use sir::global_css;
use warp::{tesseract::Tesseract, multipass::MultiPass};

use crate::components::ui_kit::{popup::Popup, icon_input::IconInput, icon_button::IconButton, button::Button};

#[derive(Props)]
pub struct Props<'a> {
    tesseract: Tesseract,
    icon: Shape,
    title: String,
    onclick: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Friends<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    // static MULTIPASS: AtomRef<Option<Arc<RwLock<Box<dyn MultiPass>>>>> = |_| None;

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
            onclick: move |_| cx.props.onclick.call(()),
            children: cx.render(rsx!(
                div {
                    class: "friends",
                    div {
                        class: "title",
                        Icon {
                            icon: cx.props.icon,
                            size: 20,
                        },
                        "{cx.props.title}",
                    },
                    label {
                        "Copy Your Friend Code",
                    },
                    div {
                        class: "add",
                        Button {
                            text: "Copy Code".to_string(),
                            icon: Shape::ClipboardCopy,
                            onclick: move |_| {
                                
                            }
                        }
                    },
                    label {
                        "Add Someone",
                    },
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