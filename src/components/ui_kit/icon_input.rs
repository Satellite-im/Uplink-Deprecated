use dioxus::{prelude::*, events::FormEvent};
use dioxus_heroicons::{outline::Shape, Icon};

#[derive(PartialEq)]
pub enum State {
    Success,
    Danger,
}

#[derive(Props)]
pub struct Props<'a> {
    icon: Shape,
    oninput: EventHandler<'a, FormEvent>,
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
pub fn IconInput<'a>(cx: Scope<'a, Props>) -> Element<'a> {
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