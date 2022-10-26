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

            // let mut stream = match multipass.subscribe() {
            //     Ok(stream) => stream,
            //     Err(_) => return,
            // };

            // while let Some(event) = stream.next().await {
            //     match event {
            //         warp::multipass::MultiPassEventKind::FriendRequestReceived { .. } => {
            //             reqCount += 1;
            //         }
            //         warp::multipass::MultiPassEventKind::FriendRequestRejected { .. }
            //         | warp::multipass::MultiPassEventKind::FriendRequestClosed { .. }
            //         | warp::multipass::MultiPassEventKind::FriendAdded { .. } => {
            //             reqCount -= 1;
            //         }
            //         _ => {}
            //     }
            // }
        },
    );

    cx.render(rsx! {
        div {
            class: "nav",
            IconButton {
                on_pressed: move |_| {
                    use_router(&cx).push_route("/main/files", None, None);
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
                (*reqCount.get() > 0).then(|| rsx!(
                    NumericIndicator {
                        count: *reqCount.get()
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
                    use_router(&cx).push_route("/main/settings", None, None);
                },
                state: icon_button::State::Secondary,
                icon: Shape::Cog
            },
        }
    })
}
