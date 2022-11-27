use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum State {
    Primary,
    Secondary,
}

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Eq, Props)]
pub struct Props {
    name: String,
    state: State,
    // Maximum amount of items something like HFS Plus could store is 2 billion items
    // Seems to align closet to the 32 bit uint range.
    children: u32,
}

#[allow(non_snake_case)]
pub fn Folder(cx: Scope<Props>) -> Element {
    let class = match cx.props.state {
        State::Primary => "primary",
        State::Secondary => "secondary",
    };

    cx.render(rsx! {
        div {
            class: "folder {class}",
            Icon { icon: Shape::Folder },
            p { "{cx.props.name}" },
            label {
                "{cx.props.children} item(s)"
            }
        }
    })
}
