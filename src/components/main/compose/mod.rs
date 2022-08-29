use dioxus::prelude::*;
use sir::global_css;
use warp::tesseract::Tesseract;

use crate::components::main::compose::write::Write;

#[derive(PartialEq, Props)]
pub struct Props {
    tesseract: Tesseract,
}

pub mod write;

#[allow(non_snake_case)]
pub fn Compose(cx: Scope) -> Element {
    global_css! ("
        .compose {
            display: inline-flex;
            flex-direction: column;
            flex: 1;

            .messages-container {
                flex: 1;
            }
            
            .writer-container {
                width: 100%;
                display: inline-flex;
            }
        }
    ");
    cx.render(rsx! {
        div {
            class: "compose",
            div {
                class: "messages-container",
            },
            div {
                class: "writer-container",
                Write {
                    onsubmit: move |_| {},
                    onupload: move |_| {},
                }
            }
        }
    })
}
