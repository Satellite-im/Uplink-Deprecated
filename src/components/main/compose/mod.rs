use dioxus::prelude::*;
use warp::tesseract::Tesseract;

use crate::components::main::compose::write::Write;

#[derive(PartialEq, Props)]
pub struct Props {
    tesseract: Tesseract,
}

pub mod write;

#[allow(non_snake_case)]
pub fn Compose(cx: Scope<Props>) -> Element {

    cx.render(rsx!{
        div {
            "Compose",
            Write {
                 submit: move |_| {},
                 upload: move |_| {},
            }
        }
    })
}