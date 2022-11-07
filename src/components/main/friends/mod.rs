use std::{collections::HashSet, time::Duration};

use arboard::Clipboard;

use dioxus::{
    core::UiEvent,
    events::{FormEvent, MouseData},
    prelude::*,
};
use dioxus_heroicons::{outline::Shape, Icon};
use dioxus_toast::{Position, ToastInfo};

use crate::{
    components::{
        main::friends::{friend::Friend, request::FriendRequest},
        ui_kit::{button::Button, icon_button::IconButton, icon_input::IconInput, popup::Popup},
    },
    Account, Messaging, LANGUAGE, TOAST_MANAGER,
};
use warp::crypto::DID;
use warp::multipass::Friends;

pub mod friend;
pub mod request;

#[derive(Props)]
pub struct Props<'a> {
    account: Account,
    messaging: Messaging,
    icon: Shape,
    title: String,
    show: bool,
    on_hide: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Friends<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {
    let toast = use_atom_ref(&cx, TOAST_MANAGER);
    let mp = cx.props.account.clone();
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let l2 = l.clone();
    let incomingRequestsLang = { l.incoming_requests.to_string() };
    let outgoingRequestsLang = { l.outgoing_requests.to_string() };
    let yourFriendsLang = { l.your_friends.to_string() };
    let codeCopied = { l.code_copied.to_string() };

    let add_error = use_state(&cx, String::new);
    let remote_friend = use_state(&cx, String::new);

    let friends = use_state(&cx, || {
        HashSet::from_iter(mp.list_friends().unwrap_or_default())
    });
    let incoming = use_state(&cx, || mp.list_incoming_request().unwrap_or_default());
    let outgoing = use_state(&cx, || mp.list_outgoing_request().unwrap_or_default());

    use_future(
        &cx,
        (friends, incoming, outgoing, &mp),
        |(friends, incoming, outgoing, mp)| async move {
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
                let friends_list: HashSet<_> =  HashSet::from_iter(mp.read().list_friends().unwrap_or_default());
                let incoming_list = mp.read().list_incoming_request().unwrap_or_default();
                let outgoing_list = mp.read().list_outgoing_request().unwrap_or_default();

                if *friends != friends_list {
                    friends.set(friends_list);
                }

                if *incoming != incoming_list {
                    incoming.set(incoming_list);
                }

                if *outgoing != outgoing_list {
                    outgoing.set(outgoing_list);
                }

                tokio::time::sleep(Duration::from_millis(300)).await;
            }
        },
    );

    cx.render(rsx! {
        Popup {
            on_dismiss: |_| cx.props.on_hide.call(()),
            hidden: !cx.props.show,
            children: cx.render(rsx!(
                div {
                    class: "friends",
                    div {
                        class: "static",
                        div {
                            class: "title",
                            Icon {
                                icon: cx.props.icon,
                                size: 20,
                            },
                            "{cx.props.title}",
                        },
                        label {
                            "{l.copy_friend_code}",
                        },
                        div {
                            class: "add",
                            Button {
                                text: l.copy_code.to_string(),
                                icon: Shape::ClipboardCopy,
                                on_pressed: move |e: UiEvent<MouseData>| {
                                    e.cancel_bubble();

                                    let mut clipboard = Clipboard::new().unwrap();
                                    if let Ok(ident) = mp
                                        .read()
                                        .get_own_identity()
                                    {
                                        let single_toast = ToastInfo {
                                            position: Position::TopRight,
                                            ..ToastInfo::simple(&codeCopied)
                                        };
                                        let _id = toast.write().popup(single_toast);
                                        clipboard.set_text(ident.did_key().to_string()).unwrap();
                                    }
                                }
                            }
                        },
                        label {
                            "{l.add_someone}",
                        },
                        div {
                            class: "add",
                            IconInput {
                                placeholder: l.add_placeholder.clone(),
                                icon: Shape::UserAdd,
                                value: remote_friend.to_string(),
                                on_change: move |evt: FormEvent| {
                                    add_error.set(String::new());
                                    remote_friend.set(evt.value.clone());
                                },
                                on_enter: move |_| {
                                    let did = DID::try_from(remote_friend.clone().to_string());
                                    match did {
                                        Ok(d) => {
                                            match cx.props.account.clone()
                                                .write()
                                                .send_request(&d)
                                            {
                                                Ok(_) => {
                                                    let single_toast = ToastInfo {
                                                        position: Position::TopRight,
                                                        ..ToastInfo::simple(l2.request_sent.clone().as_ref())
                                                    };
                                                    let _id = toast.write().popup(single_toast);
                                                    add_error.set("".into());
                                                    remote_friend.set("".into());
                                                }
                                                Err(e) => {
                                                    remote_friend.set("".into());
                                                    add_error.set(match e {
                                                        warp::error::Error::CannotSendFriendRequest => l2.couldnt_send.to_string(),
                                                        warp::error::Error::FriendRequestExist => l2.already_sent.to_string(),
                                                        warp::error::Error::CannotSendSelfFriendRequest => l2.add_self.clone(),
                                                        _ => l2.something_went_wrong.to_string()
                                                    })
                                                },
                                            };
                                        },
                                        Err(_) => add_error.set(l2.invalid_code.clone()),
                                    }
                                }
                            }
                            IconButton {
                                icon: Shape::Plus,
                                on_pressed: move |e: UiEvent<MouseData>| {
                                    e.cancel_bubble();

                                    let did = DID::try_from(remote_friend.clone().to_string());
                                    match did {
                                        Ok(d) => {
                                            match cx.props.account.clone()
                                                .write()
                                                .send_request(&d)
                                            {
                                                Ok(_) => {
                                                    let single_toast = ToastInfo {
                                                        position: Position::TopRight,
                                                        ..ToastInfo::simple(&l.request_sent)
                                                    };
                                                    let _id = toast.write().popup(single_toast);
                                                    add_error.set("".into());
                                                    remote_friend.set("".into());
                                                }
                                                Err(e) => {
                                                    remote_friend.set("".into());
                                                    add_error.set(match e {
                                                        warp::error::Error::CannotSendFriendRequest => l.couldnt_send.to_string(),
                                                        warp::error::Error::FriendRequestExist => l.already_sent.to_string(),
                                                        warp::error::Error::CannotSendSelfFriendRequest => l.add_self.to_string(),
                                                        _ => l.something_went_wrong.to_string()
                                                    })
                                                },
                                            };
                                        },
                                        Err(_) => add_error.set(l.invalid_code.to_string()),
                                    }
                                },
                            }
                        },
                        span {
                            class: "error_text",
                            "{add_error}"
                        },
                    },
                    div {
                        class: "scroll_wrap",
                        div {
                            class: "scrolling",
                            (incoming.len() > 0).then(|| rsx!(
                                label {
                                "{incomingRequestsLang}"
                                },
                                div {
                                    incoming.iter().map(|request| rsx!(
                                        FriendRequest {
                                            account: cx.props.account.clone(),
                                            request: request.clone(),
                                            on_accept: move |_| {
                                                match cx.props.account.clone()
                                                    .write()
                                                    .accept_request(&request.from())
                                                {
                                                    Ok(_) => {
                                                        add_error.set("".into());
                                                    },
                                                    Err(_) => {
                                                        // TODO: Catch this and display it
                                                        println!("Error");
                                                    },
                                                }
                                            },
                                            on_deny: move |_| {
                                                match cx.props.account.clone()
                                                    .write()
                                                    .deny_request(&request.from())
                                                {
                                                    Ok(_) => {
                                                        add_error.set("".into());
                                                    },
                                                    Err(_) => {
                                                        // TODO: Catch this and display it
                                                        println!("Error");
                                                    },
                                                }
                                            },
                                            deny_only: false,
                                        }
                                    )),
                                }
                            )),
                            (outgoing.len() > 0).then(|| rsx!(
                                label {
                                    "{outgoingRequestsLang}"
                                },
                                div {
                                    outgoing.iter().map(|request| rsx!(
                                        FriendRequest {
                                            account: cx.props.account.clone(),
                                            request: request.clone(),
                                            on_deny:  move |_| {
                                                match cx.props.account.clone()
                                                    .write()
                                                    .close_request(&request.to())
                                                {
                                                    Ok(_) => {
                                                        add_error.set("".into());
                                                    },
                                                    Err(_) => {
                                                        // TODO: Catch this and display it
                                                        println!("Error");
                                                    },
                                                }
                                            },
                                            on_accept: move |_| {},
                                            deny_only: true,
                                        }
                                    )),
                                }
                            )),
                            label {
                                "{yourFriendsLang}"
                            },
                            div {
                                friends.iter().map(|user| rsx!(
                                    Friend {
                                        account: cx.props.account.clone(),
                                        messaging: cx.props.messaging.clone(),
                                        friend: user.clone(),
                                        on_chat: move |_| {
                                            add_error.set("".into());
                                            cx.props.on_hide.call(());
                                        }
                                    }
                                )),
                            }
                        }
                    }
                }
            ))
        },
    })
}
