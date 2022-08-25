use dioxus::prelude::*;

use crate::themes::Theme;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    active: bool
}

pub fn styles() -> String {
    format!(
        "
        .switch {{
            position: relative;
            display: inline-block;
            width: 36px;
            height: 16px;
            margin-top: 2px;
        }}
        
        .switch input {{
            opacity: 0;
            width: 0;
            height: 0;
        }}
        
        .slider {{
            user-select: none;
            position: absolute;
            cursor: pointer;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background-color: {secondary};
            -webkit-transition: .2s;
            transition: .2s;
            border-radius: 10px;
        }}
        
        .slider:before {{
            position: absolute;
            content: '';
            height: 20px;
            width: 20px;
            left: 0;
            bottom: -2px;
            border-radius: 10px;
            background-color: {text_bright};
            -webkit-transition: .2s;
            transition: .2s;
        }}
        
        input:checked + .slider {{
            background-color: {primary};
        }}
        
        input:focus + .slider {{
            box-shadow: 0 0 1px {primary};
        }}
        
        input:checked + .slider:before {{
            transform: translateX(16px);
        }}
        ",
        text_bright = Theme::load_or_default().text_bright,
        primary = Theme::load_or_default().primary,
        secondary = Theme::load_or_default().secondary,
    )
}

#[allow(non_snake_case)]
pub fn Switch(cx: Scope<Props>) -> Element {
    cx.render(rsx!{
        label {
            class: "switch",
            input {
                "type": "checkbox",
                checked: "{cx.props.active}"
            },
            span {
                class: "slider",
            }
        }
    })
}