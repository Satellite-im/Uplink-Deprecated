use dioxus::{events::MouseEvent, prelude::*};
use dioxus_heroicons::{outline::Shape, Icon};

#[derive(PartialEq, Eq)]
pub enum State {
    Primary,
    Success,
    Danger,
    Secondary,
}

#[derive(Props)]
pub struct Props<'a> {
    on_pressed: EventHandler<'a, MouseEvent>,
    #[props(optional)]
    /// Text to be displayed within the button
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

    let mut class = String::from("");
    class += match cx.props.large {
        Some(_) => "button-lg",
        None => "",
    };
    class += match cx.props.state.as_ref() {
        Some(state) => match state {
            State::Success => "button-success",
            State::Danger => "button-danger",
            State::Secondary => "button-secondary",
            _ => " ",
        },
        None => "",
    };

    cx.render(match cx.props.icon {
        Some(icon) => rsx! {
            button {
                class: "button {class}",
                onclick: move |evt| cx.props.on_pressed.call(evt),
                disabled: "{disabled}",
                Icon {
                    icon: icon,
                },
                span {
                    "{text}"
                }
            }
        },
        None => rsx! {
            button {
                class: "button {class}",
                disabled: "{disabled}",
                onclick: move |evt| cx.props.on_pressed.call(evt),
                "{text}"
            }
        },
    })
}
