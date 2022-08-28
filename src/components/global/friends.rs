use std::{sync::{Arc, RwLock}, vec};

use copypasta::{ClipboardContext, ClipboardProvider};

use dioxus::{events::{FormEvent, MouseData}, prelude::*, core::UiEvent};
use dioxus_heroicons::{outline::Shape, Icon};
use dioxus_toast::{ToastInfo, Position};
use sir::global_css;

use warp::{multipass::MultiPass, tesseract::Tesseract, crypto::DID};
use warp_mp_ipfs::config::MpIpfsConfig;

use crate::{
    components::ui_kit::{
        button::Button, icon_button::IconButton, icon_input::IconInput, popup::Popup,
    },
    DEFAULT_PATH, MULTIPASS, TOAST_MANAGER,
};

#[derive(Props)]
pub struct Props<'a> {
    tesseract: Tesseract,
    icon: Shape,
    title: String,
    onclick: EventHandler<'a, ()>,
}

#[allow(non_snake_case)]
pub fn Friends<'a>(cx: Scope<'a, Props<'a>>) -> Element<'a> {    
    let toast = use_atom_ref(&cx, TOAST_MANAGER);
    let multipass = use_atom_ref(&cx, MULTIPASS);
    let mp = multipass.read().clone().unwrap().clone();


    let add_error = use_state(&cx, || "");
    let remote_friend = use_state(&cx, String::new);

    let friends = match mp.read().list_friends() {
        Ok(f) => f,
        Err(_) => vec![],
    };
    let requests = match mp.read().list_incoming_request() {
        Ok(f) => f,
        Err(_) => vec![],
    };

    global_css! {"
        .friends {
            display: inline-flex;
            flex-direction: column;

            .add {
                display: inline-flex;
                flex-direction: row;

                .icon-input {
                    width: 100%;
                    margin-right: 1rem;
                }
            }
        }
    "};

    cx.render(rsx! {
        Popup {
            on_dismiss: |_| cx.props.onclick.call(()),
            children: cx.render(rsx!(
                div {
                    class: "friends",
                    div {
                        class: "title",
                        Icon {
                            icon: cx.props.icon,
                            size: 20,
                        },
                        "{cx.props.title}",
                    },
                    label {
                        "Copy Your Friend Code",
                    },
                    div {
                        class: "add",
                        Button {
                            text: "Copy Code".to_string(),
                            icon: Shape::ClipboardCopy,
                            on_pressed: move |e: UiEvent<MouseData>| {
                                e.cancel_bubble();

                                let mut ctx = ClipboardContext::new().unwrap();
                                let contents = match multipass
                                        .read()
                                        .clone()
                                        .unwrap()
                                        .write()
                                        .get_own_identity()
                                    {
                                        Ok(ident) => {
                                            ident.did_key().to_string()
                                        }
                                        Err(_) => "".to_string(),
                                    };
                                let single_toast = ToastInfo {
                                    position: Position::TopRight,
                                    ..ToastInfo::simple("Copied your code!")
                                };
                                let _id = toast.write().popup(single_toast);
                                ctx.set_contents(contents).unwrap();
                            }
                        }
                    },
                    label {
                        "Add Someone",
                    },
                    span {
                        class: "error_text",
                        "{add_error}"
                    },
                    div {
                        class: "add",
                        IconInput {
                            placeholder: "Warp#a3fdc6..".to_string(),
                            icon: Shape::UserAdd,
                            value: remote_friend.to_string(),
                            on_change: move |evt: FormEvent| remote_friend.set(evt.value.clone()),
                            on_enter: move |_| {
                                let did = DID::try_from(remote_friend.clone().to_string());
                                match did {
                                    Ok(d) => {
                                        match multipass
                                            .read()
                                            .clone()
                                            .unwrap()
                                            .write()
                                            .send_request(&d)
                                        {
                                            Ok(_) => {
                                                let single_toast = ToastInfo {
                                                    position: Position::TopRight,
                                                    ..ToastInfo::simple("Friend request sent!")
                                                };
                                                let _id = toast.write().popup(single_toast);
                                                add_error.set("".into());
                                                remote_friend.set("".into());
                                            }
                                            Err(e) => {
                                                remote_friend.set("".into());
                                                add_error.set(match e {
                                                    warp::error::Error::CannotSendFriendRequest => "Couldn't send friend request.",
                                                    warp::error::Error::FriendRequestExist => "You've already sent this request.",
                                                    warp::error::Error::CannotSendSelfFriendRequest => "You cannot add yourself as a friend.",
                                                    _ => "Something went wrong."
                                                })
                                            },
                                        };
                                    },
                                    Err(_) => add_error.set("Invalid friend code.".into()),
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
                                        match multipass
                                            .read()
                                            .clone()
                                            .unwrap()
                                            .write()
                                            .send_request(&d)
                                        {
                                            Ok(_) => {
                                                let single_toast = ToastInfo {
                                                    position: Position::TopRight,
                                                    ..ToastInfo::simple("Friend request sent!")
                                                };
                                                let _id = toast.write().popup(single_toast);
                                                add_error.set("".into());
                                                remote_friend.set("".into());
                                            }
                                            Err(e) => {
                                                remote_friend.set("".into());
                                                add_error.set(match e {
                                                    warp::error::Error::CannotSendFriendRequest => "Couldn't send friend request.",
                                                    warp::error::Error::FriendRequestExist => "You've already sent this request.",
                                                    warp::error::Error::CannotSendSelfFriendRequest => "You cannot add yourself as a friend.",
                                                    _ => "Something went wrong."
                                                })
                                            },
                                        };
                                    },
                                    Err(_) => add_error.set("Invalid friend code.".into()),
                                }
                            },
                        }
                    },
                    if requests.len() > 0 {
                        rsx!(
                            label {
                                "Incoming Requests"
                            },
                            div {
                                requests.iter().map(|_request| rsx!(
                                    div {
                                        "request"
                                    }
                                )),
                            }
                        )
                    } else {
                        rsx!(span {})
                    },
                    label {
                        "Your Friends"
                    },
                    div {
                        friends.iter().map(|_request| rsx!(
                            div {
                                "friend"
                            }
                        )),
                    }
                }
            ))
        },
    })
}
