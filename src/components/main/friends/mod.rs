pub mod friend;
pub mod request;
pub mod sidebar;

use crate::{
    components::{
        main::friends::{friend::Friend, sidebar::Sidebar},
        ui_kit::icon_button::IconButton,
    },
    Account, Messaging,
};

use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use std::{collections::HashSet, time::Duration};
use warp::multipass::Friends;

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Friends(cx: Scope<Props>) -> Element {
    let add_error = use_state(&cx, String::new);
    let friends = use_state(&cx, || {
        HashSet::from_iter(cx.props.account.list_friends().unwrap_or_default())
    });

    use_future(
        &cx,
        (friends, &cx.props.account.clone()),
        |(friends, mp)| async move {
            // todo: use this commented out code somehow. i assume it's being saved for later.
            // mp is of type Account
            // let mut stream = match mp.subscribe() {
            //     Ok(stream) => stream,
            //     Err(_) => return,
            // };

            // while let Some(event) = stream.next().await {
            //     match event {
            //         warp::multipass::MultiPassEventKind::FriendRequestReceived { .. } => {
            //             incoming.set(mp.list_incoming_request().unwrap_or_default());
            //         }
            //         warp::multipass::MultiPassEventKind::FriendRequestRejected { .. } => {
            //             incoming.set(mp.list_incoming_request().unwrap_or_default());
            //         }
            //         warp::multipass::MultiPassEventKind::FriendRequestClosed { .. } => {
            //             incoming.set(mp.list_incoming_request().unwrap_or_default());
            //             outgoing.set(mp.list_incoming_request().unwrap_or_default());
            //         }
            //         warp::multipass::MultiPassEventKind::FriendAdded { did } => {
            //             if mp.has_friend(&did).is_ok() {
            //                 friends.needs_update();
            //             }
            //         }
            //         warp::multipass::MultiPassEventKind::FriendRemoved { did } => {
            //             if mp.has_friend(&did).is_err() {
            //                 friends.needs_update();
            //             }
            //         }
            //         _ => {}
            //     }
            // }

            loop {
                let friends_list: HashSet<_> =
                    HashSet::from_iter(mp.read().list_friends().unwrap_or_default());

                if *friends != friends_list {
                    friends.set(friends_list);
                }

                tokio::time::sleep(Duration::from_millis(300)).await;
            }
        },
    );

    cx.render(rsx! {
        div {
            class: "friends-container app-container",
            Sidebar { account: cx.props.account.clone(), add_error: add_error.clone()},
            div {
                class: "friends-main app-main",
                div {
                    class: "toolbar",
                    div {
                        class: "controls",
                        IconButton {
                            icon: Shape::X,
                            on_pressed: move |_| {
                                use_router(&cx).push_route("/main", None, None);
                            }
                        }
                    }
                },
                div {
                    class: "friends-list",
                    friends.iter().map(|user| rsx!(
                        Friend {
                            account: cx.props.account.clone(),
                            messaging: cx.props.messaging.clone(),
                            friend: user.clone(),
                            on_chat: move |_| {
                                add_error.set("".into());
                                use_router(&cx).push_route("/main", None, None);
                            }
                        }
                    )),
                }
            }
        }
    })
}
