use dioxus::{events::MouseEvent, prelude::*};
use dioxus_heroicons::{outline::Shape, Icon};

#[derive(PartialEq, Eq)]
pub enum State {
    Primary,
    Secondary,
    Success,
    Danger,
    Filled,
    Transparent,
}

#[derive(Props)]
pub struct Props<'a> {
    on_pressed: EventHandler<'a, MouseEvent>,
    #[props(optional)]
    text: Option<String>,
    #[props(optional)]
    icon: Option<Shape>,
    #[props(optional)]
    large: Option<bool>,
    #[props(optional)]
    state: Option<State>,
    #[props(optional)]
    disabled: Option<bool>,
}

#[allow(non_snake_case)]
pub fn Button<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    let disabled = cx.props.disabled.unwrap_or(false);

    let text = match cx.props.text.clone() {
        Some(t) => t,
        None => String::from(""),
    };

    let mut class = String::from("button ");
    class += match cx.props.large {
        Some(_) => "button-lg ",
        None => "",
    };
    class += match cx.props.state.as_ref() {
        Some(state) => match state {
            State::Secondary => "button-secondary",
            State::Success => "button-success",
            State::Danger => "button-danger",
            State::Filled => "button-filled",
            State::Transparent => "button-transparent",
            _ => "",
        },
        None => "",
    };
    // add class if text length is 0
    if text.len() == 0 {
        class += " button-icon-only";
    }

    cx.render(rsx! {
        button {
            class: "{class} ellipsis",
            disabled: "{disabled}",
            onclick: move |evt| cx.props.on_pressed.call(evt),
            cx.render(match cx.props.icon {
                Some(icon) => rsx! {
                    Icon {
                        icon: icon,
                    }
                },
                None => rsx! {Fragment()},
            }),
            div {
                class: "button-text ellipsis",
                "{text}"
            }
        }
    })
}
