use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};

#[derive(PartialEq)]
pub enum State {
    Success,
    Danger,
}

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    icon: Shape,
    placeholder: String,
    #[props(optional)]
    text: Option<String>,
}


pub fn css() -> String {"
    .icon-input {
        position: relative;
    }
    .icon-input .input {
        padding-left: 40px;
    }
    .icon-input svg {
        position: absolute;
        z-index: 2;
        stroke: var(--theme-placeholder);
        top: 11px;
        left: 10px;
        fill: transparent;
    }
    .icon-input:has(> input:focus) svg {
        stroke: var(--theme-primary) !important;
    }".to_string()}

#[allow(non_snake_case)]
pub fn IconInput(cx: Scope<Props>) -> Element {
    cx.render(rsx!{
            div {
                class: "icon-input",
                Icon {
                    icon: cx.props.icon,
                },
                input {
                    class: "input",
                    placeholder: "{cx.props.placeholder}"
                },
            }
        }
    )
}