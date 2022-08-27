use dioxus::prelude::*;
use sir::global_css;
use warp::tesseract::Tesseract;

use crate::components::ui_kit::popup::Popup;

#[derive(PartialEq, Props)]
pub struct Props {
    tesseract: Tesseract,
}

#[allow(non_snake_case)]
pub fn Friends(cx: Scope<Props>) -> Element {
    global_css! {"
        .friends {
            
        }
    "}

    cx.render(rsx!{
        Popup {
            tesseract: cx.props.tesseract.clone(),
            children: cx.render(rsx!(
                span {
                    "Friends"
                }
            ))
        },
    })
}