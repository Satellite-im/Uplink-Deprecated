use dioxus::{events::FormEvent, prelude::*};

#[derive(PartialEq)]
pub enum State {
    Success,
    Danger,
}

#[derive(Props)]
pub struct Props<'a> {
    placeholder: String,
    on_change: EventHandler<'a, FormEvent>,
}

#[allow(non_snake_case)]
pub fn Input<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    cx.render(rsx! {
        input {
            class: "input",
            placeholder: "{cx.props.placeholder}",
            oninput: |evt| cx.props.on_change.call(evt),
        }
    })
}
