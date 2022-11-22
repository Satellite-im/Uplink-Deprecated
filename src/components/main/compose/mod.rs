pub mod messages;
pub mod msg;
pub mod reply;
pub mod topbar;
pub mod write;

use crate::{
    components::{
        main::compose::{messages::Messages, topbar::TopBar, write::Write},
        ui_kit::icon_button::IconButton,
        ui_kit::users_typing_indicator::UsersTypingIndicator,
    },
    state::{Actions, LastMsgSent},
    Account, Messaging, LANGUAGE, STATE,
};
use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use warp::raygun::RayGun;
use warp::raygun::TypingIndicator;

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

    cx.render(rsx! {
        div {
            class: "compose",
                rsx!(
                    TopBar {
                        account: cx.props.account.clone(),
                        on_call: move |_| {},
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
                    div {
                        class: "messages-container",
                        Messages {
                            account: cx.props.account.clone(),
                            messaging: cx.props.messaging.clone(),
                        }
                    },
                    Write {
                        on_submit: move |message: String| {
                            text.set(String::from(""));
                            let mut rg = cx.props.messaging.clone();

                            let text_as_vec = message
                                .split('\n')
                                .filter(|&s| !s.is_empty())
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>();

                            if text_as_vec.is_empty() {
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

                                // TODO: We need to wire this message up to display differently
                                // until we confim whether it was successfully sent or failed
                                if let Err(_e) = warp::async_block_in_place_uncheck(rg.send(id, None, text_as_vec)) {
                                    //TODO: Handle error
                                };
                            }
                        },
                        on_trigger_typing: move |typing: TypingIndicator| {
                            let mut rg = cx.props.messaging.clone();
                            if let Some(id) = current_chat {
                                println!("typing: {:?}", typing);
                                if let Err(_e) = warp::async_block_in_place_uncheck(rg.indicate_typing(id, typing)) {
                                }
                            }
                        },
                        on_upload: move |_| {}
                    }
                    UsersTypingIndicator{
                        account: cx.props.account,
                        current_chat: current_chat
                    }
                )
        }
    })
}
