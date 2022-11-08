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
    Account, Language, Messaging, LANGUAGE, TOAST_MANAGER,
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
    let l = use_atom_ref(&cx, LANGUAGE).read().clone();
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
    let incoming = use_state(&cx, || {
        HashSet::from_iter(mp.list_incoming_request().unwrap_or_default())
    });
    let outgoing = use_state(&cx, || {
        HashSet::from_iter(mp.list_outgoing_request().unwrap_or_default())
    });

    use_future(
        &cx,
        (friends, incoming, outgoing, &mp),
        |(friends, incoming, outgoing, mp)| async move {
            loop {
                let friends_list: HashSet<_> =
                    HashSet::from_iter(mp.read().list_friends().unwrap_or_default());
                let incoming_list: HashSet<_> =
                    HashSet::from_iter(mp.read().list_incoming_request().unwrap_or_default());
                let outgoing_list: HashSet<_> =
                    HashSet::from_iter(mp.read().list_outgoing_request().unwrap_or_default());

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

    // occurs when a friend is selected via IconInput or IconButton
    let select_friend = move |l: Language| {
        let did = DID::try_from((*remote_friend.current()).clone());
        remote_friend.set("".into());
        match did {
            Ok(d) => {
                match cx.props.account.clone().write().send_request(&d) {
                    Ok(_) => {
                        let single_toast = ToastInfo {
                            position: Position::TopRight,
                            ..ToastInfo::simple(&l.request_sent)
                        };
                        let _id = toast.write().popup(single_toast);
                        if !(*add_error.current()).is_empty() {
                            add_error.set(String::new());
                        }
                    }
                    Err(e) => add_error.set(match e {
                        warp::error::Error::CannotSendFriendRequest => l.couldnt_send.to_string(),
                        warp::error::Error::FriendRequestExist => l.already_sent.to_string(),
                        warp::error::Error::CannotSendSelfFriendRequest => l.add_self.to_string(),
                        _ => l.something_went_wrong.to_string(),
                    }),
                };
            }
            Err(_) => add_error.set(l.invalid_code.to_string()),
        }
    };
    let select_friend2 = select_friend.clone();

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
                                on_change: move |evt: FormEvent| {
                                    if !(*add_error.current()).is_empty() {
                                        add_error.set(String::new());
                                    }
                                    remote_friend.set(evt.value.clone());
                                },
                                on_enter: move |_| {
                                    select_friend(l.clone());
                                }
                            }
                            IconButton {
                                icon: Shape::Plus,
                                on_pressed: move |e: UiEvent<MouseData>| {
                                    e.cancel_bubble();
                                    select_friend2(l2.clone());
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
                            (!incoming.is_empty()).then(|| rsx!(
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
                            (!outgoing.is_empty()).then(|| rsx!(
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
