use dioxus::prelude::*;

pub enum ResizeDirection {
    Horizontal,
    Vertical,
}

#[derive(Props)]
pub struct Props<'a> {
    direction: ResizeDirection,
    children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Resizable<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let class = format!(
        "resize-container {}",
        match cx.props.direction {
            ResizeDirection::Horizontal => "horizontal",
            ResizeDirection::Vertical => "vertical",
        }
    );

    let script = include_str!("resizable.js");

    cx.render(rsx! {
        div {
            class: "{class}",
            &cx.props.children,
            div {
                class: "resize-handle",
            }
            script { "{script}" }
        }
    })
}
