use dioxus::prelude::*;
use sir::global_css;
use warp::tesseract::Tesseract;

#[derive(Props)]
pub struct Props<'a> {
    tesseract: Tesseract,
    children: Element<'a>
}

#[allow(non_snake_case)]
pub fn Popup<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    global_css! {"
        .popup-mask {
            backdrop-filter: blur(10px);
            position: fixed;
            left: 0;
            right: 0;
            bottom: 0;
            left: 0;
            .popup {
                min-width: calc(100% - 2rem);
                min-height: calc(100% - 2rem);
                margin: 2rem;
                background: var(--theme-background-light);
            }
        }
    "}

    cx.render(rsx!(
        div {
            class: "popup-mask",
            div {
                class: "popup",
                cx.props.children.as_ref()
            }
        }
    ))
}