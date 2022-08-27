use crate::{main::sidebar::Sidebar, components::{main::compose::Compose, global::friends::Friends}};
use dioxus::prelude::*;
use sir::global_css;
use warp::{tesseract::Tesseract};

pub mod sidebar;
pub mod compose;

#[derive(PartialEq, Props)]
pub struct Props {
    tesseract: Tesseract,
}

#[allow(non_snake_case)]
pub fn Main(cx: Scope<Props>) -> Element {
    // Start UI
    global_css! {"
    .main {
        display: flex;
        justify-content: center;
        align-items: center;
        text-align: center;
        width: 100;
        height: 100;
    }
    "}

    cx.render(rsx!{
        div {
            class: "main",
            Sidebar {
                tesseract: cx.props.tesseract.clone()
            },
            Compose {
                tesseract: cx.props.tesseract.clone()
            },
            Friends {
                tesseract: cx.props.tesseract.clone()
            }
        }
    })
}