pub mod messages;
pub mod msg;
pub mod reply;
pub mod topbar;
pub mod write;

use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use ui_kit::{icon_button::IconButton, typing_indicator::TypingIndicator};

use rfd::FileDialog;
use std::path::PathBuf;
use warp::crypto::DID;

use crate::{
    components::{
        main::compose::{messages::Messages, topbar::TopBar, write::Write},
        media::MediaContainer,
    },
    state::{Actions, LastMsgSent},
    Messaging, LANGUAGE, STATE,
};
use ::utils::Account;

#[derive(PartialEq, Props)]
pub struct Props {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Compose(cx: Scope<Props>) -> Element {
    log::debug!("rendering Compose");
    let state = use_atom_ref(&cx, STATE);
    let current_chat = state.read().current_chat;
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let warningMessage = l.prerelease_warning.to_string();
    let text = use_state(&cx, String::new);
    let show_warning = use_state(&cx, || true);
    let show_media = use_state(&cx, || false);
    let users_typing: &UseRef<HashMap<DID, String>> = use_ref(&cx, HashMap::new);

    let selected_file = use_state(&cx, || -> Option<Vec<PathBuf>> { None });
    let selected_file_str = &selected_file
        .clone()
        .as_ref()
        .unwrap_or(&vec![])
        .iter()
        .map(|f| f.clone().into_os_string().into_string().unwrap_or_default())
        .collect::<Vec<String>>()
        .join(", ");

    cx.render(rsx! {
        div {
            class: "compose",
                rsx!(
                    TopBar {
                        account: cx.props.account.clone(),
                        on_call: move |_| {
                            show_media.set(!show_media);
                        },
                    },
                    (**show_warning).then(|| rsx!(
                        div {
                            class: "alpha-warning animate__animated animate__slideInDown",
                            "{warningMessage}",
                            IconButton {
                                on_pressed: move |_| {
                                    show_warning.set(false);
                                },
                                icon: Shape::Check,
                            }
                        },
                    )),
                    (**show_media).then(|| rsx! {
                        MediaContainer {
                            account: cx.props.account.clone(),
                        }
                    }),
                    div {
                        class: "messages-container",
                        Messages {
                            account: cx.props.account.clone(),
                            messaging: cx.props.messaging.clone(),
                            users_typing: users_typing.clone(),
                        }
                    },
                    div {
                        "{selected_file_str}"
                    },
                    Write {
                        messaging: cx.props.messaging.clone(),
                        on_submit: move |message: String| {
                            text.set(String::from(""));
                            let mut rg = cx.props.messaging.clone();

                            let text_as_vec = message
                                .split('\n')
                                .filter(|&s| !s.is_empty())
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>();

                            if text_as_vec.is_empty() && selected_file.is_none() {
                                return;
                            }

                            // clicking the send button is meaningless if there isn't a conversation.
                            if let Some(id) = current_chat {

                                // mutate the state
                                let cur = state.read().all_chats.get(&id).cloned();
                                if let Some( mut conversation_info) = cur {
                                    conversation_info.last_msg_sent = Some(LastMsgSent::new(&text_as_vec));
                                    state.write().dispatch(Actions::UpdateConversation(conversation_info));
                                }

                                if selected_file.is_some() {
                                    let attachments = selected_file.as_ref().unwrap().to_vec();
                                    if let Err(_e) = warp::async_block_in_place_uncheck(rg.attach(id, attachments, text_as_vec)) {
                                        //TODO: Handle error
                                        println!("Error: {:?}", _e);
                                    }
                                    selected_file.set(None);
                                } else {
                                    if let Err(_e) = warp::async_block_in_place_uncheck(rg.send(id, None, text_as_vec)) {
                                        //TODO: Handle error
                                        println!("Error: {:?}", _e);
                                    };
                                };
                                // TODO: We need to wire this message up to display differently
                                // until we confim whether it was successfully sent or failed

                            }
                        },
                        on_upload: move |_| {
                            let file = FileDialog::new()
                                .set_directory("/")
                                .pick_files();
                            selected_file.set(file);
                        }
                    },
                    TypingIndicator{
                        users: users_typing.clone()
                    }
                )
        }
    })
}
