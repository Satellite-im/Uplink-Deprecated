use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::components::ui_kit::icon_button::{self, IconButton};

pub enum NavEvent {
    Home,
    Files,
    Friends,
    Profile,
}

#[derive(Props)]
pub struct Props<'a> {
    on_pressed: EventHandler<'a, NavEvent>,
}

#[allow(non_snake_case)]
pub fn Nav<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    cx.render(rsx! {
        div {
            class: "nav",
            IconButton {
                on_pressed: move |_| {
                    let _ = &cx.props.on_pressed.call(NavEvent::Files);
                },
                state: icon_button::State::Secondary,
                icon: Shape::Folder
            },
            IconButton {
                on_pressed: move |_| {
                    let _ = &cx.props.on_pressed.call(NavEvent::Friends);
                },
                state: icon_button::State::Secondary,
                icon: Shape::Users
            },
            IconButton {
                on_pressed: move |_| {
                    let _ = &cx.props.on_pressed.call(NavEvent::Profile);
                },
                state: icon_button::State::Secondary,
                icon: Shape::UserCircle
            },
            IconButton {
                on_pressed: move |_| {
                   //  let _ = &cx.props.on_pressed.call(NavEvent::Profile);
                },
                state: icon_button::State::Secondary,
                icon: Shape::Cog
            },
        }
    })
}
