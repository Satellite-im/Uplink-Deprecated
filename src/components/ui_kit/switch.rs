use dioxus::{core::UiEvent, events::FormData, prelude::*};

// Remember: owned props must implement PartialEq!
#[derive(Props)]
pub struct Props<'a> {
    active: bool,
    // TODO: we should insted just return a bool with the switche's binary state
    on_change: EventHandler<'a, UiEvent<FormData>>,
}

#[allow(non_snake_case)]
pub fn Switch<'a>(cx: Scope<'a, Props>) -> Element<'a> {
    cx.render(rsx! {
        label {
            class: "switch",
            input {
                "type": "checkbox",
                checked: "{cx.props.active}",
                oninput: move |evt| cx.props.on_change.call(evt)
            },
            span {
                class: "slider",
            }
        }
    })
}
