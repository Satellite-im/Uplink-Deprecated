use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use ui_kit::icon_button::IconButton;

#[allow(non_snake_case)]
pub fn Controls(cx: Scope) -> Element {
    log::debug!("rendering Media Controls");

    cx.render(rsx! {
        div {
            id: "media-controls",
            IconButton {
                icon: Shape::Microphone,
                state: ui_kit::icon_button::State::Secondary,
                on_pressed: |_| {}
            },
            IconButton {
                icon: Shape::VideoCamera,
                state: ui_kit::icon_button::State::Secondary,
                on_pressed: |_| {}
            },
            IconButton {
                icon: Shape::ComputerDesktop,
                state: ui_kit::icon_button::State::Secondary,
                on_pressed: |_| {}
            },
            IconButton {
                icon: Shape::PhoneXMark,
                state: ui_kit::icon_button::State::Danger,
                on_pressed: |_| {}
            },
        }
    })
}
