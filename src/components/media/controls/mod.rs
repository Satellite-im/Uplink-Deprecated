use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use ui_kit::button::Button;

#[allow(non_snake_case)]
pub fn Controls(cx: Scope) -> Element {
    log::debug!("rendering Media Controls");

    cx.render(rsx! {
        div {
            id: "media-controls",
            // button for microphone
            Button {
                icon: Shape::Microphone,
                state: ui_kit::button::State::Secondary,
                on_pressed: |_| {}
            },
            // button for video camera
            Button {
                icon: Shape::VideoCamera,
                state: ui_kit::button::State::Secondary,
                on_pressed: |_| {}
            },
            // button for window
            Button {
                icon: Shape::Window,
                state: ui_kit::button::State::Secondary,
                on_pressed: |_| {}
            },
            // button to end call
            Button {
                icon: Shape::PhoneXMark,
                state: ui_kit::button::State::Danger,
                on_pressed: |_| {}
            },
        }
    })
}
