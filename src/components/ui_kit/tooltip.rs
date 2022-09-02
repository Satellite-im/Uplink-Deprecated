use dioxus::prelude::*;

#[derive(PartialEq)]
pub enum State {
    Secondary,
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

#[allow(non_snake_case)]
pub fn Tooltip(cx: Scope<Props>) -> Element {
    let text = match cx.props.text.clone() {
        Some(t) => t,
        None => String::from(""),
    };

    let mut class = String::from("");
    class += match &cx.props.state {
        Some(s) => match s {
            State::Secondary => "tooltip tooltip-secondary ",
        },
        None => "tooltip ",
    };
    class += match cx.props.arrow_position.as_ref() {
        Some(position) => match position {
            ArrowPosition::TopLeft => "tooltip-arrow-top-left ",
            ArrowPosition::Top => "tooltip-arrow-top ",
            ArrowPosition::TopRight => "tooltip-arrow-top-right ",
            ArrowPosition::Left => "tooltip-arrow-left ",
            ArrowPosition::Right => "tooltip-arrow-right ",
            ArrowPosition::BottomLeft => "tooltip-arrow-bottom-left ",
            ArrowPosition::Bottom => "tooltip-arrow-bottom ",
            ArrowPosition::BottomRight => "tooltip-arrow-bottom-right ",
        },
        None => "tooltip-arrow-bottom",
    };

    cx.render(rsx! {
        div {
            button {
                class: "{class}",
                "{text}"
            }
        }
    })
}
