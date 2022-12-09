use dioxus::prelude::*;

use ui_kit::button::Button;

#[derive(Props)]
pub struct ListTypeButtonProps<'a> {
    text: String,
    active: bool,
    on_pressed: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn ListTypeButton<'a>(cx: Scope<'a, ListTypeButtonProps<'a>>) -> Element<'a> {
    let class = if cx.props.active {
        "active"
    } else {
        "inactive"
    };

    cx.render(rsx!(
        div {
            class: "list-type-button {class}",
            Button {
                on_pressed: move |_| cx.props.on_pressed.call(()),
                text: cx.props.text.clone()
            }
        }
    ))
}
