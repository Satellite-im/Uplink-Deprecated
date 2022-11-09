pub mod friend;
pub mod request;
pub mod sidebar;

use crate::{
    components::{
        main::friends::{self, friend::Friend},
        reusable::{self, nav::NavEvent},
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
            reusable::sidebar::Sidebar { account: cx.props.account.clone(), active: NavEvent::Friends, friends::sidebar::Sidebar{account: cx.props.account.clone(), add_error: add_error.clone()}},
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
    })
}
