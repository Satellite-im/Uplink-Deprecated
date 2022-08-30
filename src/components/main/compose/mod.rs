use dioxus::prelude::*;
use sir::global_css;
use warp::{tesseract::Tesseract, crypto::DID};

use crate::components::main::compose::{write::Write, topbar::TopBar};

#[derive(PartialEq, Props)]
pub struct Props {
    did: DID,
}

pub mod write;
pub mod topbar;

#[allow(non_snake_case)]
pub fn Compose(cx: Scope<Props>) -> Element {
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
            TopBar {
                did: cx.props.did.clone(),
                on_call: move |_| {},
            },
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
