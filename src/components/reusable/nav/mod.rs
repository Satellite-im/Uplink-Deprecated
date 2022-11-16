use crate::utils::notifications::PushNotification;
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use futures::StreamExt;

use crate::{
    components::ui_kit::{
        icon_button::{self, IconButton},
        numeric_indicator::NumericIndicator,
    },
    Account,
};
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
            // Used to make sure everything is initalized before proceeding.
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
                        let _name_or_did = match multipass
                            .get_identity(from.clone().into())
                            .ok()
                            .and_then(|list| list.first().cloned())
                            .map(|id| id.username())
                        {
                            Some(name) => name,
                            None => from.to_string(),
                        };
                        PushNotification(
                            "New Friend Request".to_owned(),
                            "Come see who it is!".to_owned(),
                            // format!("{:#?} sent a friend request", most_recent_friend_request),
                            crate::utils::sounds::Sounds::FriendReq,
                        );
                        log::debug!("updating friend request count");
                        let count = *(reqCount.get()) + 1;
                        // Note, this will increase the counter. Maybe use a seperate task to check the list or use other events to decrease it
                        reqCount.set(count);
                    }
                    MultiPassEventKind::FriendRequestRejected { .. } => {
                        log::debug!("updating friend request count");
                        let count = *(reqCount.get()) - 1;
                        reqCount.set(count);
                    }
                    // left this commented out to allow for one to determine the logic here
                    // MultiPassEventKind::FriendRequestClosed { from, .. } => {
                    //     log::debug!("updating friend request count");
                    //     let count = *(reqCount.get()) - 1;
                    //     reqCount.set(count);
                    // }
                    // left commented out in the event one wants to use different logic or changes here
                    // MultiPassEventKind::FriendAdded { did } => {
                    //     let name_or_did = match multipass
                    //         .get_identity(did.clone().into())
                    //         .ok()
                    //         .and_then(|list| list.first().cloned())
                    //         .map(|id| id.username())
                    //     {
                    //         Some(name) => name,
                    //         None => did.to_string(),
                    //     };

                    //     PushNotification(
                    //         "New Friend".to_owned(),
                    //         format!("You are now friends with {name_or_did}"),
                    //         // format!("{:#?} sent a friend request", most_recent_friend_request),
                    //         crate::utils::sounds::Sounds::FriendReq,
                    //     );
                    //     log::debug!("updating friend request count");
                    //     let count = *(reqCount.get()) - 1;
                    //     reqCount.set(count);
                    // }
                    _ => {}
                }
            }

            // loop {
            //     let list = multipass.list_incoming_request().unwrap_or_default();
            //     if list.len() != *reqCount.get() {
            //         // If list is updated, we update the request count
            //         if list.len() > *reqCount.get() {
            //             // We display a notification if the list length is increased (incoming request appended).
            //             // We do not display a notification if the list length is decreased (incoming request is rejected).

            //             PushNotification(
            //                 "New Friend Request".to_owned(),
            //                 "Come see who it is!".to_owned(),
            //                 // format!("{:#?} sent a friend request", most_recent_friend_request),
            //                 crate::utils::sounds::Sounds::FriendReq,
            //             );
            //         }

            //         reqCount.set(list.len());
            //     }
            //     tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            // }
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
