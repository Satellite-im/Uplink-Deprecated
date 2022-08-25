use dioxus::prelude::*;

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

pub fn css() -> String {"
    .input {
        min-width: 100px;
        height: 40px;
        padding-left: 10px;
        padding-right: 10px;
        color: var(--theme-text);
        border-radius: 4px;
        border: none;
        background: var(--theme-secondary);
    }
    .input:focus {
        outline: none;
        box-sizing: border-box;
        border: 1px solid  var(--theme-primary);
        margin: 1px;
    }
    ".to_string()}

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