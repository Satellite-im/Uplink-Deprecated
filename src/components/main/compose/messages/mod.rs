use crate::{components::main::compose::msg::Msg, state::Actions, Account, Messaging, STATE};
use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};

use futures::StreamExt;
use warp::raygun::{MessageEventKind, MessageOptions, RayGun, RayGunStream};

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Messages(cx: Scope<Props>) -> Element {
    //Note: We will just unwrap for now though we need to
    //      handle the error properly if there is ever one when
    //      getting own identity
    let state = use_atom_ref(&cx, STATE).clone();
    // this needs to be passed to the use_future to make the messages page reload when a new chat is selected.
    let ext_conversation_id = state.read().current_chat;
    let ident = cx.props.account.read().get_own_identity().unwrap();
    let messages = use_ref(&cx, Vec::new);

    use_future(
        &cx,
        (messages, &cx.props.messaging.clone(), &ext_conversation_id),
        |(list, mut rg, input_conversation_id)| async move {
            // don't stream messages from a nonexistent conversation
            let mut current_chat = match input_conversation_id {
                // this better not panic
                Some(id) => state.read().all_chats.get(&id).cloned().unwrap(),
                None => return,
            };

            let mut stream = loop {
                match rg
                    .get_conversation_stream(current_chat.conversation.id())
                    .await
                {
                    Ok(stream) => break stream,
                    Err(e) => match &e {
                        warp::error::Error::RayGunExtensionUnavailable => {
                            //Give sometime for everything in the background to fully line up
                            //Note, if this error still happens, it means there is an fatal error
                            //      in the background
                            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                        }
                        _ => {
                            // todo: properly report this error
                            // eprintln!("failed to get_conversation_stream: {}", e);
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        }
                    },
                }
            };
            let messages = rg
                .get_messages(current_chat.conversation.id(), MessageOptions::default())
                .await
                .unwrap_or_default();

            //This is to prevent the future updating the state and causing a rerender
            if *list.read() != messages {
                // assumes the most recent message is first in the list
                if let Some(msg) = messages.first() {
                    current_chat.last_msg_read = Some(msg.id());
                    state
                        .write_silent()
                        .dispatch(Actions::UpdateConversation(current_chat.clone()))
                        .save();
                }

                *list.write() = messages;
            }

            while let Some(event) = stream.next().await {
                match event {
                    MessageEventKind::MessageReceived {
                        conversation_id,
                        message_id,
                    }
                    | MessageEventKind::MessageSent {
                        conversation_id,
                        message_id,
                    } => {
                        if current_chat.conversation.id() == conversation_id {
                            if let Ok(message) = rg.get_message(conversation_id, message_id).await {
                                current_chat.last_msg_read = Some(message.id());
                                state
                                    .write_silent()
                                    .dispatch(Actions::UpdateConversation(current_chat.clone()))
                                    .save();
                                list.write().push(message);
                            }
                        }
                    }
                    _ => {}
                }
            }
        },
    );

    cx.render({
        let mut prev_sender = "".to_string();
        rsx! {
            div {
                class: "messages",
                messages.read().iter().rev().map(|message|{
                    let msg_sender = message.sender().to_string();
                    let i = ident.did_key().to_string();
                    let remote = i != msg_sender;
                    let last = prev_sender != msg_sender;
                    let middle = prev_sender == msg_sender;
                    let first = false;

                    prev_sender = message.sender().to_string();

                    rsx!(
                        Msg {
                            message: message.clone(),
                            remote: remote,
                            last: last,
                            first: first,
                            middle: middle,
                        }
                    )
                })
                div {
                    class: "encrypted-notif",
                    Icon {
                        icon: Shape::LockClosed
                    }
                    p {
                        "Messages secured by local E2E encryption."
                    }
                }
            }
        }
    })
}
