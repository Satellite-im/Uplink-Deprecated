use dioxus::prelude::*;

use crate::themes::Theme;

#[derive(PartialEq)]
pub enum State {
    Secondary
}

#[derive(PartialEq)]
pub enum ArrowPosition {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

// Remember: owned props must implement PartialEq!
#[derive(PartialEq, Props)]
pub struct Props {
    #[props(optional)]
    text: Option<String>,
    #[props(optional)]
    state: Option<State>,
    #[props(optional)]
    arrow_position: Option<ArrowPosition>,
}

pub fn styles() -> String {
    format!(
        "
        .tooltip {{
            user-select: none;
            height: 35px;
            min-width: 80px;
            border-radius: 4px;
            border: none;
            color: {text_bright};
            background-color: {primary};
            position: relative;
        }}
        .tooltip-arrow-top, .tooltip-arrow-top-left, .tooltip-arrow-top-right {{
            margin-top: 8px;
        }}
        .tooltip-arrow-bottom, .tooltip-arrow-bottom-left, .tooltip-arrow-bottom-right {{
            margin-bottom: 8px;
        }}
        .tooltip-arrow-top-left::after {{
            content: '';
            position: absolute;
            bottom: 100%;
            left: 16px;
            margin-left: -6px;
            border-width: 8px;
            border-style: solid;
            border-color: transparent transparent {primary} transparent;
        }}
        .tooltip-arrow-top::after {{
            content: '';
            position: absolute;
            bottom: 100%;
            left: 50%;
            margin-left: -6px;
            border-width: 8px;
            border-style: solid;
            border-color: transparent transparent {primary} transparent;
        }}
        .tooltip-arrow-top-right::after {{
            content: '';
            position: absolute;
            bottom: 100%;
            right: 16px;
            margin-left: -6px;
            border-width: 8px;
            border-style: solid;
            border-color: transparent transparent {primary} transparent;
        }}
        .tooltip-arrow-left::after {{
            content: '';
            position: absolute;
            bottom: calc(50% - 8px);
            right: 100%;
            margin-left: -6px;
            border-width: 8px;
            border-style: solid;
            border-color: transparent {primary} transparent transparent;
        }}
        .tooltip-arrow-right::after {{
            content: '';
            position: absolute;
            bottom: calc(50% - 8px);
            left: 100%;
            margin-right: -6px;
            border-width: 8px;
            border-style: solid;
            border-color: transparent transparent transparent {primary};
        }}
        .tooltip-arrow-bottom-left::after {{
            content: '';
            position: absolute;
            top: 100%;
            left: 16px;
            margin-left: -6px;
            border-width: 8px;
            border-style: solid;
            border-color: {primary} transparent transparent transparent;
        }}
        .tooltip-arrow-bottom::after {{
            content: '';
            position: absolute;
            top: 100%;
            left: 50%;
            margin-left: -6px;
            border-width: 8px;
            border-style: solid;
            border-color: {primary} transparent transparent transparent;
        }}
        .tooltip-arrow-bottom-right::after {{
            content: '';
            position: absolute;
            top: 100%;
            right: 16px;
            margin-left: -6px;
            border-width: 8px;
            border-style: solid;
            border-color: {primary} transparent transparent transparent;
        }}
        .tooltip-secondary {{
            background-color: {secondary};
        }}
        .tooltip-arrow-right.tooltip-secondary::after {{
            border-color: transparent transparent transparent {secondary};
        }}
        .tooltip-arrow-bottom-left.tooltip-secondary::after,
        .tooltip-arrow-bottom-right.tooltip-secondary::after,
        .tooltip-arrow-bottom.tooltip-secondary {{
            border-color: {secondary} transparent transparent transparent;
        }}
        .tooltip-arrow-left.tooltip-secondary::after {{
            border-color: transparent {secondary} transparent transparent;
        }}
        .tooltip-arrow-top-right.tooltip-secondary::after,
        .tooltip-arrow-top.tooltip-secondary::after,
        .tooltip-arrow-top-left.tooltip-secondary::after {{
            border-color: transparent transparent {secondary} transparent;
        }}
        ",
        text_bright = Theme::load_or_default().text_bright,
        primary = Theme::load_or_default().primary,
        secondary = Theme::load_or_default().secondary,
    )
}

#[allow(non_snake_case)]
pub fn Tooltip(cx: Scope<Props>) -> Element {
    let text = match cx.props.text.clone() {
        Some(t) => t,
        None => String::from(""),
    };

    let mut class = String::from("");
    class += match &cx.props.state {
        Some(s) => {
            match s {
                State::Secondary => "tooltip tooltip-secondary ",
            }
        },
        None => "tooltip ",
    };
    class += match cx.props.arrow_position.as_ref() {
        Some(position ) => {
            match position {
                ArrowPosition::TopLeft => "tooltip-arrow-top-left ",
                ArrowPosition::Top => "tooltip-arrow-top ",
                ArrowPosition::TopRight => "tooltip-arrow-top-right ",
                ArrowPosition::Left => "tooltip-arrow-left ",
                ArrowPosition::Right => "tooltip-arrow-right ",
                ArrowPosition::BottomLeft => "tooltip-arrow-bottom-left ",
                ArrowPosition::Bottom => "tooltip-arrow-bottom ",
                ArrowPosition::BottomRight => "tooltip-arrow-bottom-right ",
            }
        },
        None => "tooltip-arrow-bottom",
    };

    cx.render(rsx!{
        div {
            button {
                class: "{class}",
                "{text}"
            }
        }
    })
}