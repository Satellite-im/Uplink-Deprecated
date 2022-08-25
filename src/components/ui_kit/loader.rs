use dioxus::prelude::*;

use crate::themes::Theme;

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    text: Option<String>
}

pub fn styles() -> String {
    format!(
        "
        .bar {{
            float: left;
            width: 15px;
            height: 6px;
            border-radius: 2px;
            background-color: {text_bright};
        }}
        .load .bar {{
            animation: loadingJ 2s cubic-bezier(0.17, 0.37, 0.43, 0.67) infinite;
        }}
        .load p {{
            color: {placeholder};
            animation: loadingK 2s cubic-bezier(0.17, 0.37, 0.43, 0.67) infinite;
            font-family: 'Space Mono', monospace;
        }}
        @keyframes loadingK {{
            0%,
            100% {{
                color: {placeholder};
            }}
            50% {{
                color: {secondary};
            }}
        }}
        @keyframes loadingJ {{
            0%,
            100% {{
                transform: translate(0, 0);
            }}
            50% {{
                transform: translate(80px, 0);
                background-color: {secondary};
                width: 25px;
            }}
        }}
        ",
        text_bright = Theme::load_or_default().text_bright,
        placeholder = Theme::load_or_default().placeholder,
        secondary = Theme::load_or_default().secondary,
    )
}

#[allow(non_snake_case)]
pub fn Loader(cx: Scope<Props>) -> Element {
    cx.render(rsx! {
        div {
            class: "load",
            p {
                class: "",
                cx.props.text.clone()
            },
            div {
                class: "bar"
            }
        }
    })
}