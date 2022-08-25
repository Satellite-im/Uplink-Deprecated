use dioxus::prelude::*;

use crate::themes::Theme;


#[derive(PartialEq)]
pub enum InputState {
    Success,
    Danger,
}


// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct InputProps {
    placeholder: String,
    #[props(optional)]
    text: Option<String>,
}

pub fn styles() -> String {
    format!(
        "
        .input {{
            min-width: 100px;
            height: 40px;
            padding-left: 10px;
            padding-right: 10px;
            color: {text};
            border-radius: 4px;
            border: none;
            background: {secondary}
        }}
        .input:focus {{
            outline: none;
            box-sizing: border-box;
            border: 1px solid {primary};
            margin: 1px;
        }}
        ",
        primary = Theme::load_or_default().primary,
        secondary = Theme::load_or_default().secondary,
        text = Theme::load_or_default().text
    )
}

#[allow(non_snake_case)]
pub fn Input(cx: Scope<InputProps>) -> Element {
    cx.render(rsx!{
            input {
                class: "input",
                placeholder: "{cx.props.placeholder}"
            }
        }
    )
}