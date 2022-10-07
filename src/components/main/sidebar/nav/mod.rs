use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::{
    components::{
        ui_kit::icon_button::{self, IconButton},
    },
    Account,
};

pub enum NavEvent {
    Home,
    Files,
    Friends,
    Profile,
    Settings,
}

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    on_pressed: EventHandler<'a, NavEvent>,
}

#[allow(non_snake_case)]
pub fn Nav<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    // Total incoming request count
    let requests = use_state(&cx, Vec::new);
    requests.set(cx.props.account.read().list_incoming_request().unwrap_or_default());
    let reqCount: usize = requests.len();
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
            span {
                IconButton {
                    on_pressed: move |_| {
                        let _ = &cx.props.on_pressed.call(NavEvent::Friends);
                    },
                    state: icon_button::State::Secondary,
                    icon: Shape::Users
                },
                (reqCount > 0).then(|| rsx!(
                    span {
                        class: "pill",
                        "{reqCount}"
                    }
                )),
            }
            IconButton {
                on_pressed: move |_| {
                    let _ = &cx.props.on_pressed.call(NavEvent::Profile);
                },
                state: icon_button::State::Secondary,
                icon: Shape::UserCircle
            },
            IconButton {
                on_pressed: move |_| {
                    let _ = &cx.props.on_pressed.call(NavEvent::Settings);
                },
                state: icon_button::State::Secondary,
                icon: Shape::Cog
            },
        }
    })
}
