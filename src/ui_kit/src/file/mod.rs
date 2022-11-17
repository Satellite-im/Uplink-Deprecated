use dioxus::prelude::*;
use dioxus_heroicons::{solid::Shape, Icon};

use super::folder::State;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Eq, Props)]
pub struct Props {
    name: String,
    state: State,
    kind: String,
    size: u64,
}

#[allow(non_snake_case)]
pub fn File(cx: Scope<Props>) -> Element {
    let class = match cx.props.state {
        State::Primary => "primary",
        State::Secondary => "secondary",
    };

    cx.render(rsx! {
        div {
            class: "folder {class}",
            Icon { icon: Shape::Document },
            p { "{cx.props.name}" },
            label {
                "{cx.props.size} MiB"
            }
        }
    })
}
