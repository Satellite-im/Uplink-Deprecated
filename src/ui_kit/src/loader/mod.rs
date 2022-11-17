use dioxus::prelude::*;

#[derive(PartialEq, Eq, Props)]
pub struct Props {
    text: Option<String>,
}

#[allow(non_snake_case)]
pub fn Loader(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            class: "load",
            span {
                cx.props.text.clone()
            },
            div {
                class: "bar"
            }
        }
    })
}
