use dioxus::prelude::*;

#[derive(PartialEq, Eq)]
pub enum Size {
    Large,
    Normal,
    Small,
}

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Eq, Props)]
pub struct Props {
    src: String,
    size: Size,
}

#[allow(non_snake_case)]
pub fn PFP(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            class: "pfp",
            style: "background-image: url({cx.props.src});"
        }
    })
}
