use dioxus::{events::MouseEvent, prelude::*};
use dioxus_heroicons::{outline::Shape, Icon};

#[derive(PartialEq, Eq)]
pub enum State {
    Primary,
    Secondary,
    Success,
    Danger,
}

#[derive(Props)]
pub struct Props<'a> {
    icon: Shape,
    on_pressed: EventHandler<'a, MouseEvent>,
    #[props(optional)]
    large: Option<bool>,
    #[props(optional)]
    state: Option<State>,
    #[props(optional)]
    text: Option<String>,
    #[props(optional)]
    disabled: Option<bool>,
}

#[allow(non_snake_case)]
pub fn IconButton<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    let disabled = cx.props.disabled.unwrap_or(false);

    let mut class = String::from("");
    class += match cx.props.large {
        Some(_) => "button icon-button icon-button-lg ",
        None => "button icon-button ",
    };
    class += match cx.props.state.as_ref() {
        Some(state) => match state {
            State::Success => "button-success ",
            State::Danger => "button-danger ",
            State::Secondary => "button-secondary",
            _ => " ",
        },
        None => "",
    };

    cx.render(rsx! {
        button {
            class: "{class}",
            onclick: move |evt| cx.props.on_pressed.call(evt),
            disabled: "{disabled}",
            Icon {
                icon: cx.props.icon,
            },
            cx.props.text.clone().map(|text| rsx!("{text}")),
        }
    })
}
