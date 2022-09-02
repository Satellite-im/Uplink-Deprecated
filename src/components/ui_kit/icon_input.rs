use dioxus::{
    events::{FormEvent, KeyCode},
    prelude::*,
};
use dioxus_heroicons::{outline::Shape, Icon};

#[derive(PartialEq)]
pub enum State {
    Success,
    Danger,
}

#[derive(Props)]
pub struct Props<'a> {
    icon: Shape,
    value: String,
    on_change: EventHandler<'a, FormEvent>,
    on_enter: EventHandler<'a, ()>,
    placeholder: String,
}

#[allow(non_snake_case)]
pub fn IconInput<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "icon-input",
            Icon {
                icon: cx.props.icon,
            },
            input {
                class: "input",
                placeholder: "{cx.props.placeholder}",
                oninput: move |e| cx.props.on_change.call(e),
                value: "{cx.props.value}",
                onkeyup: move |evt| {
                    if evt.key_code == KeyCode::Enter {
                        cx.props.on_enter.call(())
                    }
                }
            },
        }
    })
}
