use ::utils::notifications::PushNotification;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use futures::StreamExt;
use ui_kit::{
    icon_button::{self, IconButton},
    numeric_indicator::NumericIndicator,
};

use crate::{Account, LANGUAGE};
use warp::multipass::{Friends, FriendsEvent, MultiPass, MultiPassEventKind};

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
    let l = use_atom_ref(&cx, LANGUAGE).read().clone();
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
        |(reqCount, mut multipass)| async move {
            // Used to make sure everything is initialized before proceeding.
            let new_friend_request_notification = l.new_friend_request.to_string().to_owned();

            let mut stream = loop {
                match multipass.subscribe() {
                    Ok(stream) => break stream,
                    Err(e) => match e {
                        //Note: Used as a precaution for future checks
                        warp::error::Error::MultiPassExtensionUnavailable => {
                            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                        }
                        //TODO: log error
                        //Note: Shouldnt give any other error but if it does to probably file as a bug
                        _ => return,
                    },
                };
            };

            // one can process other events such as event sent, closed, accepted, etc
            // Note: should only use events related to `FriendRequest` here
            while let Some(event) = stream.next().await {
                match event {
                    MultiPassEventKind::FriendRequestReceived { from } => {
                        // Use to show the name or did of who its from
                        let name_or_did = multipass
                            .get_identity(from.clone().into())
                            .ok()
                            .and_then(|list| list.first().cloned())
                            .map(|id| id.username())
                            .unwrap_or_else(|| from.to_string());

                        PushNotification(
                            new_friend_request_notification.clone(),
                            // "New Friend Request".to_owned(),
                            format!("{} sent a friend request", name_or_did),
                            // "Come see who it is!".to_owned(),
                            ::utils::sounds::Sounds::FriendReq,
                        );
                        log::debug!("updating friend request count");
                        // Note, this will increase the counter. Maybe use a separate task to check the list or use other events to decrease it
                        reqCount.with_mut(|count| *count += 1);
                    }
                    MultiPassEventKind::IncomingFriendRequestRejected { .. } => {
                        log::debug!("updating friend request count");
                        if *(reqCount.get()) != 0 {
                            log::debug!("close friend request");
                            reqCount.with_mut(|count| *count -= 1);
                        }
                    }
                    MultiPassEventKind::IncomingFriendRequestClosed { .. } => {
                        log::debug!("updating friend request count");
                        if *(reqCount.get()) != 0 {
                            log::debug!("close friend request");
                            reqCount.with_mut(|count| *count -= 1);
                        }
                    }
                    MultiPassEventKind::FriendAdded { .. } => {
                        log::debug!("updating friend request count");
                        if *(reqCount.get()) != 0 {
                            reqCount.with_mut(|count| *count -= 1);
                        }
                    }
                    _ => {}
                }
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
