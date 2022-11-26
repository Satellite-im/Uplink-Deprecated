use dioxus::prelude::*;

#[derive(PartialEq, Eq, Props)]
pub struct Props {
    #[props(optional)]
    large: Option<bool>,
}

#[allow(non_snake_case)]
pub fn InlineSkeleton(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            class: "inline-skeleton",
        }
    })
}
