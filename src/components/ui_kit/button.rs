use dioxus::prelude::*;
use dioxus_heroicons::{Icon, outline::Shape};

use crate::themes::Theme;


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

pub fn styles() -> String {
    format!(
        "
        .button {{
            user-select: none;
            cursor: pointer;
            background-color: {primary};
            border-radius: 20px;
            height: 40px;
            min-width: 110px;
            text-align: center;
            border: none;
            gap: 8px;
            color: {text_bright};
            stroke: {text_bright};
        }}
        .button:hover {{
            background-color: {primary_light};
        }}
        .button:active {{
            background-color: {primary};
        }}
        .button-secondary {{
            background-color: {secondary};
        }}
        .button-secondary:hover {{
            background-color: {secondary_light};
        }}
        .button-secondary:active {{
            background-color: {secondary};
        }}
        .button span {{
            display: inline-block;
            margin-bottom: 8px;
        }}
        .button svg {{
            margin-bottom: -5px;
            padding-top: 5px;
            margin-right: 5px;
        }}
        .button-lg {{
            height: 52px;
            border-radius: 26px;
        }}
        .button-success {{
            background-color: {green};
        }}
        .button-success:hover {{
            background-color: {light_green};
        }}
        .button-success:active {{
            background-color: {green};
        }}
        .button-danger {{
            background-color: {red};
        }}
        .button-danger:hover {{
            background-color: {light_red};
        }}
        .button-danger:active {{
            background-color: {red};
        }}
        ",
        text_bright = Theme::load_or_default().text_bright,
        primary = Theme::load_or_default().primary,
        primary_light = Theme::load_or_default().primary_light,
        secondary = Theme::load_or_default().secondary,
        secondary_light = Theme::load_or_default().secondary_light,
        red = Theme::load_or_default().red,
        green = Theme::load_or_default().green,
        light_green = Theme::load_or_default().light_green,
        light_red = Theme::load_or_default().light_red,
    )
}

#[allow(non_snake_case)]
pub fn Button(cx: Scope<Props>) -> Element {
    let text = match cx.props.text.clone() {
        Some(t) => t,
        None => String::from(""),
    };

    let mut class = String::from("");
    class += match cx.props.large {
        Some(_) => "button button-lg ",
        None => "button",
    };
    class += match cx.props.state.as_ref() {
        Some(state) => {
            match state {
                State::Secondary => "button0-secondary",
                State::Success => "button-success ",
                State::Danger => "button-danger ",
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