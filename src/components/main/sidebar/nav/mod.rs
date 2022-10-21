use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::{
    components::ui_kit::{
        icon_button::{self, IconButton},
        numeric_indicator::NumericIndicator,
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
    requests.set(
        cx.props
            .account
            .read()
            .list_incoming_request()
            .unwrap_or_default(),
    );
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
            div {
                class: "has_indicator",
                IconButton {
                    on_pressed: move |_| {
                        let _ = &cx.props.on_pressed.call(NavEvent::Friends);
                    },
                    state: icon_button::State::Secondary,
                    icon: Shape::Users
                },
                (reqCount > 0).then(|| rsx!(
                    NumericIndicator {
                        count: reqCount
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
                    use_router(&cx).push_route("/settings", None, None);
                },
                state: icon_button::State::Secondary,
                icon: Shape::Cog
            },
        }
    })
}
