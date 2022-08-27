use dioxus::prelude::*;
use warp::tesseract::Tesseract;

#[derive(PartialEq, Props)]
pub struct Props {
    tesseract: Tesseract,
}

pub mod write;

#[allow(non_snake_case)]
pub fn Compose(cx: Scope<Props>) -> Element {

    cx.render(rsx!{
        div {
            "Compose"
        }
    })
}