use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;

use crate::{
    components::ui_kit::{
        icon_button::{self, IconButton},
        numeric_indicator::NumericIndicator,
    },
    Account,
};
use warp::multipass::Friends;

#[derive(PartialEq, Eq, Clone, Copy)]
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
    active: NavEvent,
    on_pressed: EventHandler<'a, NavEvent>,
}

#[allow(non_snake_case)]
pub fn Nav<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let multipass = cx.props.account.clone();
    let reqCount = use_state(&cx, || {
        multipass.list_incoming_request().unwrap_or_default().len()
    });

    use_future(
        &cx,
        (reqCount, &multipass),
        |(reqCount, multipass)| async move {
            loop {
                let list = multipass.list_incoming_request().unwrap_or_default();
                if list.len() != *reqCount.get() {
                    reqCount.set(list.len());
                }
                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            }
        },
    );

    cx.render(rsx! {
        div {
            class: "nav",
            IconButton {
                on_pressed: move |_| {
                    use_router(&cx).push_route("/main", None, None);
                },
                state: if cx.props.active.eq(&NavEvent::Home) {
                    icon_button::State::Primary
                } else {
                    icon_button::State::Secondary
                }
                icon: Shape::Chat
            },
            IconButton {
                on_pressed: move |_| {
                    use_router(&cx).push_route("/main/files", None, None);
                },
                state: if cx.props.active.eq(&NavEvent::Files) {
                    icon_button::State::Primary
                } else {
                    icon_button::State::Secondary
                },
                icon: Shape::Folder
            },
            div {
                class: "has_indicator",
                IconButton {
                    on_pressed: move |_| {
                        use_router(&cx).push_route("/main/friends", None, None);
                    },
                    state: if cx.props.active.eq(&NavEvent::Friends) {
                        icon_button::State::Primary
                    } else {
                        icon_button::State::Secondary
                    }
                    icon: Shape::Users
                },
                (*reqCount.get() > 0).then(|| rsx!(
                    NumericIndicator {
                        count: *reqCount.get()
                    }
                )),
            }
            IconButton {
                on_pressed: move |_| {
                    use_router(&cx).push_route("/main/settings", None, None);
                },
                state: if cx.props.active.eq(&NavEvent::Settings) {
                    icon_button::State::Primary
                } else {
                    icon_button::State::Secondary
                },
                icon: Shape::Cog
            },
        }
    })
}
