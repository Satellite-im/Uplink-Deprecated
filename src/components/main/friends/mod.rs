
pub mod friend;
pub mod request;
pub mod sidebar;

use crate::{
    components::{
        main::friends::{{sidebar::Sidebar, friend::Friend}, request::FriendRequest},
        ui_kit::{button::Button, icon_button::IconButton, icon_input::IconInput, popup::Popup},
    },
    Account, Messaging, LANGUAGE, TOAST_MANAGER,
};

use arboard::Clipboard;
use dioxus::{
    core::UiEvent,
    events::{FormEvent, MouseData},
    prelude::*,
};
use dioxus_heroicons::{outline::Shape, Icon};
use dioxus_toast::{Position, ToastInfo};
use std::{collections::HashSet, time::Duration};
use warp::{crypto::DID};
use warp::multipass::Friends;



#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Friends(cx: Scope<Props>) -> Element {
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let add_error = use_state(&cx, String::new);
    let friends = use_state(&cx, || {
        HashSet::from_iter(cx.props.account.list_friends().unwrap_or_default())
    });
    let friendString = l.friends.to_string();
    let yourFriendsLang = { l.your_friends.to_string() };
   

    use_future(
        &cx,
        (friends, &cx.props.account.clone()),
        |(friends, mp)| async move {
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
            id: "friends",
            Sidebar { account: cx.props.account.clone(), add_error: add_error.clone()},
            div {
                id: "content",
                div {
                    class: "title",
                    Icon {
                        icon: Shape::Users,
                        size: 20,
                    },
                    "{friendString}",
                },
                FindFriends { account: cx.props.account.clone(), add_error: add_error.clone()},
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
                                use_router(&cx).push_route("/main", None, None);
                            }
                        }
                    )),
                }
            }
        }
    })
}

#[inline_props]
#[allow(non_snake_case)]
pub fn FindFriends(cx: Scope, account: Account, add_error: UseState<String>) -> Element {
    let toast = use_atom_ref(&cx, TOAST_MANAGER);
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let remote_friend = use_state(&cx, String::new);


    let l2 = l.clone();
    let codeCopied = { l.code_copied.to_string() };
    let account2 = account.clone();

    cx.render(rsx!(
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
                    if let Ok(ident) = account2
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
                            match account.clone()
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
                            match account.clone()
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
    ))
}