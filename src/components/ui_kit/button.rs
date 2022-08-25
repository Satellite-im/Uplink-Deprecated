use dioxus::prelude::*;
use dioxus_heroicons::{Icon, outline::Shape};

#[derive(PartialEq)]
pub enum State {
    Success,
    Danger,
    Secondary,
}

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    #[props(optional)]
    /// Text to be displayed within the button
    text: Option<String>,
    #[props(optional)]
    icon: Option<Shape>,
    #[props(optional)]
    large: Option<bool>,
    #[props(optional)]
    state: Option<State>,
}

pub fn css() -> String {"
    .button {
        user-select: none;
        cursor: pointer;
        background-color: var(--theme-primary);
        border-radius: 20px;
        height: 40px;
        min-width: 110px;
        text-align: center;
        border: none;
        gap: 8px;
        color: var(--theme-text-bright);
        stroke: var(--theme-text-bright);
    }
    .button:hover {
        background-color: var(--theme-primary-light);
    }
    .button:active {
        background-color: var(--theme-primary);
    }
    .button-secondary {
        background-color: var(--theme-secondary);
    }
    .button-secondary:hover {
        background-color: var(--theme-secondary-light);
    }
    .button-secondary:active {
        background-color: var(--theme-secondary);
    }
    .button span {
        display: inline-block;
        margin-bottom: 8px;
    }
    .button svg {
        margin-bottom: -5px;
        padding-top: 5px;
        margin-right: 5px;
    }
    .button-lg {
        height: 52px;
        border-radius: 26px;
    }
    .button-success {
        background-color: var(--theme-green);
    }
    .button-success:hover {
        background-color: var(--theme-light-green);
    }
    .button-success:active {
        background-color: var(--theme-green);
    }
    .button-danger {
        background-color: var(--theme-red);
    }
    .button-danger:hover {
        background-color: var(--theme-light-red);
    }
    .button-danger:active {
        background-color: var(--theme-red);
    }
    ".to_string()}


#[allow(non_snake_case)]
pub fn Button(cx: Scope<Props>) -> Element {

    let text = match cx.props.text.clone() {
        Some(t) => t,
        None => String::from(""),
    };

    let mut class = String::from("");
    class += match cx.props.large {
        Some(_) => "button button-lg ",
        None => "button ",
    };
    class += match cx.props.state.as_ref() {
        Some(state) => {
            match state {
                State::Success => "button-success ",
                State::Danger => "button-danger ",
                State::Secondary => "button-secondary "
            }
        },
        None => "",
    };

    cx.render(match cx.props.icon {
        Some(icon ) => rsx!{
            div {
                button {
                    class: "{class}",
                    Icon {
                        icon: icon,
                    },
                    span {
                        "{text}"
                    }
                }
            }
        },
        None => rsx!{
            div {
                button {
                    class: "{class}",
                    "{text}"
                }
            }
        },
    })
}