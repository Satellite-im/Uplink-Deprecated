use dioxus::prelude::*;

#[inline_props]
#[allow(non_snake_case)]
pub fn Sidebar(cx: Scope, _account: crate::Account) -> Element {
    cx.render(rsx! {
        div {
            id: "sidebar",
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
                }
            }
        },
    })
}
