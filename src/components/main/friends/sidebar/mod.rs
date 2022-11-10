
use crate::{
    components::{
        main::friends::{ request::FriendRequest},
        ui_kit::{button::Button, icon_button::IconButton, icon_input::IconInput},
    },
    Account,  LANGUAGE, TOAST_MANAGER,
};

use arboard::Clipboard;
use dioxus::{
    core::UiEvent,
    events::{FormEvent, MouseData},
    prelude::*,
};
use dioxus_heroicons::{outline::Shape};
use dioxus_toast::{Position, ToastInfo};

use std::{collections::HashSet, time::Duration};
use warp::multipass::Friends;
use warp::crypto::DID;

#[inline_props]
#[allow(non_snake_case)]
pub fn Sidebar(cx: Scope, account: Account, add_error: UseState<String>) -> Element {
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let incomingRequestsLang = { l.incoming_requests.to_string() };
    let outgoingRequestsLang = { l.outgoing_requests.to_string() };

    let incoming: &UseState<HashSet<_>> = use_state(&cx, || {
        HashSet::from_iter(account.list_incoming_request().unwrap_or_default())
    });
    let outgoing: &UseState<HashSet<_>> = use_state(&cx, || {
        HashSet::from_iter(account.list_outgoing_request().unwrap_or_default())
    });

    use_future(
        &cx,
        (incoming, outgoing, &account.clone()),
        |(incoming, outgoing, account)| async move {
            loop {
                let incoming_list: HashSet<_> =
                    HashSet::from_iter(account.read().list_incoming_request().unwrap_or_default());
                let outgoing_list: HashSet<_> =
                    HashSet::from_iter(account.read().list_outgoing_request().unwrap_or_default());

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

    cx.render(rsx!(
        crate::components::reusable::sidebar::Sidebar {
            account: cx.props.account.clone(),
            FindFriends { account: account.clone(), add_error: add_error.clone()},
            div {
                class: "scroll_wrap",
                div {
                    class: "scrolling",
                    (!incoming.is_empty()).then(|| rsx!(
                        label {
                        "{incomingRequestsLang}"
                        },
                        div {
                            incoming.iter().map(|request| rsx!(
                                FriendRequest {
                                    account: account.clone(),
                                    request: request.clone(),
                                    on_accept: move |_| {
                                        match account.clone()
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
                                        match account.clone()
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
                    (!outgoing.is_empty()).then(|| rsx!(
                        label {
                            "{outgoingRequestsLang}"
                        },
                        div {
                            outgoing.iter().map(|request| rsx!(
                                FriendRequest {
                                    account: account.clone(),
                                    request: request.clone(),
                                    on_deny:  move |_| {
                                        match account.clone()
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
                }
            }
        }
    ))
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
            class: "sidebar-section",
            label {
                "{l.copy_friend_code}",
            },
            div {
                class: "code",
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
                            //copy to the clipboard without prefix 'did:key:'
                             clipboard.set_text(&ident.did_key().to_string()[8..]).unwrap();
                        }
                    }
                }
            }
        },
        label {
            "{l.add_someone}",
        },
        div {
            class: "sidebar-section",
            label {
                "{l.add_someone}",
            },
            div {
                class: "add",
                IconInput {
                    placeholder: l.add_placeholder.clone(),
                    icon: Shape::UserAdd,
                    on_change: move |evt: FormEvent| {
                        add_error.set(String::new());
                        remote_friend.set(evt.value.clone());
                    },
                    on_enter: move |_| {
                        let did = DID::try_from(String::from("did:key:") + &(remote_friend.clone().to_string()));                
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
                        let did = DID::try_from(String::from("did:key:") + &(remote_friend.clone().to_string()));           
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
        }
    ))
}