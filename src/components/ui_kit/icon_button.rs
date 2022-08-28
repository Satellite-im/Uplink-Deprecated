use dioxus::{prelude::*, events::MouseEvent};
use dioxus_heroicons::{Icon, outline::Shape};

#[derive(PartialEq)]
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
    disabled: Option<bool>
}

pub fn css() -> String {"
    .icon-button {
        user-select: none;
        min-width: 40px;
        padding: 0;
    }
    .icon-button svg {
        margin-bottom: 0;
        margin-right: 0;
        fill: transparent;
    }
    .icon-button-lg {
        min-width: 52px;
        height: 52px;
        border-radius: 26px;
    }
    ".to_string()}

#[allow(non_snake_case)]
pub fn IconButton<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    let disabled = if cx.props.disabled.is_some() { true } else { false };

    let mut class = String::from("");
    class += match cx.props.large {
        Some(_) => "button icon-button icon-button-lg ",
        None => "button icon-button ",
    };
    class += match cx.props.state.as_ref() {
        Some(state) => {
            match state {
                State::Success => "button-success ",
                State::Danger => "button-danger ",
                State::Secondary => "button-secondary",
                _ => " "
            }
        },
        None => "",
    };

    cx.render(rsx!{
        div {
            style: "max-width: 40px; display: inline-block;",
            button {
                class: "{class}",
                onclick: move |evt| cx.props.on_pressed.call(evt),
                disabled: "{disabled}",
                Icon {
                    icon: cx.props.icon,
                },
                cx.props.text.clone().map(|text| rsx!("{text}")),
            }
        }
    })
}