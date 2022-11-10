use dioxus::prelude::*;

#[derive(Props)]
pub struct Props<'a> {
    children: Element<'a>,
    controls: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Toolbar<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            id: "toolbar",
            &cx.props.children,
            div {
                id: "controls",
                &cx.props.controls
            }
        },
    })
}
