use crate::utils::{get_username_from_did, notifications::PushNotification};
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

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}

#[allow(non_snake_case)]
pub fn Nav(cx: Scope<Props>) -> Element {
    log::debug!("rendering reusable Nav");
    let multipass = cx.props.account.clone();
    let reqCount = use_state(&cx, || {
        multipass.list_incoming_request().unwrap_or_default().len()
    });

    let route = use_route(&cx).last_segment();

    let active = match route {
        Some(r) => match r {
            "main" => NavEvent::Home,
            "files" => NavEvent::Files,
            "friends" => NavEvent::Friends,
            "settings" => NavEvent::Settings,
            _ => NavEvent::Home,
        },
        None => todo!(),
    };

    use_future(
        &cx,
        (reqCount, &multipass),
        |(reqCount, multipass)| async move {
            loop {
                let list = multipass.list_incoming_request().unwrap_or_default();
                if list.len() != *reqCount.get() {
                    // If list is updated, we update the request count
                    if list.len() > *reqCount.get() {
                        // We display a notification if the list length is increased (incoming request appended).
                        // We do not display a notification if the list length is decreased (incoming request is rejected).

                        // TODO: Display sender name of incoming friend request
                        let most_recent_friend_request = list.last().unwrap().from(); // This returns a FriendRequest struct
                                                                                      // TODO: Get username from DID: get_username_from_did(most_recent_friend_request, &mp);
                                                                                      // Where &mp is a lifetime. In other file it is usually: let mp = &cx.props.account.clone();
                                                                                      // Lifetime issue:
                                                                                      // lifetime may not live long enough
                                                                                      // returning this value requires that `'1` must outlive `'static`

                        let _username =  // does this address the TODO? 
                            get_username_from_did(most_recent_friend_request, &multipass);
                        PushNotification(
                            "New Friend Request".to_owned(),
                            "Come see who it is!".to_owned(),
                            // format!("{:#?} sent a friend request", most_recent_friend_request),
                            "Friend Request".to_owned(),
                        );
                    }
                    log::debug!("updating friend request count");
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
                state: if active.eq(&NavEvent::Home) {
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
                state: if active.eq(&NavEvent::Files) {
                    icon_button::State::Primary
                } else {
                    icon_button::State::Secondary
                },
                icon: Shape::Folder
            },
            div {
                class: {
                    if active.eq(&NavEvent::Friends) {
                        format_args!("has_indicator parent_active")
                    } else {
                        format_args!("has_indicator")
                    }
                },
                IconButton {
                    on_pressed: move |_| {
                        use_router(&cx).push_route("/main/friends", None, None);
                    },
                    state: if active.eq(&NavEvent::Friends) {
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
                state: if active.eq(&NavEvent::Settings) {
                    icon_button::State::Primary
                } else {
                    icon_button::State::Secondary
                },
                icon: Shape::Cog
            },
        }
    })
}
