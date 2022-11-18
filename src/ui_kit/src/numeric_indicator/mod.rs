use dioxus::prelude::*;

#[derive(PartialEq, Eq, Props)]
pub struct Props {
    count: usize,
}

#[allow(non_snake_case)]
pub fn NumericIndicator(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        span {
            class: "numeric_indicator",
            "{cx.props.count}"
        }
    })
}
