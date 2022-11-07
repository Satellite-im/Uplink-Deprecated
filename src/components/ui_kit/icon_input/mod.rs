use dioxus::{events::FormEvent, prelude::*};
use dioxus_heroicons::{outline::Shape, Icon};
use dioxus_html::KeyCode;

#[derive(PartialEq, Eq)]
pub enum State {
    Success,
    Danger,
}

#[derive(Props)]
pub struct Props<'a> {
    icon: Shape,
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
                oninput: |e: FormEvent| {
                    cx.props.on_change.call(e);
                },
                onkeyup: |evt| {
                    if evt.key_code == KeyCode::Enter {
                        cx.props.on_enter.call(())
                    }
                }
            },
        }
    })
}
