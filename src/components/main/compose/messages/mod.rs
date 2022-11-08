use std::collections::VecDeque;

use crate::{
    components::main::compose::{msg::Msg, reply::Reply},
    state::Actions,
    Account, Messaging, STATE,
};
use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};

use futures::StreamExt;
use warp::raygun::{Message, MessageEventKind, MessageOptions, RayGun, RayGunStream};

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

    let mut rg = cx.props.messaging.clone();
    let ident = cx.props.account.read().get_own_identity().unwrap();
    // this one has a special name because of the other variable names within the use_future
    let list: UseRef<Vec<Message>> = use_ref(&cx, Vec::new).clone();
    // this one is for the rsx! macro
    let messages = list.clone();

    let current_chat = state
        .read()
        .current_chat
        .and_then(|x| state.read().all_chats.get(&x).cloned());

    // restart the use_future when the current_chat changes
    use_future(&cx, &current_chat, |current_chat| async move {
        // don't stream messages from a nonexistent conversation
        let mut current_chat = match current_chat {
            // this better not panic
            Some(c) => c,
            None => return,
        };

        if current_chat.num_unread_messages != 0 {
            current_chat.num_unread_messages = 0;
            state
                .write_silent()
                .dispatch(Actions::UpdateConversation(current_chat.clone()));
        }

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
                            list.write().push(message);
                        }
                    }
                }
                _ => {}
            }
        }
    });

    let rg = cx.props.messaging.clone();
    let messages_vec = VecDeque::from(
        messages
            .read()
            .clone()
            .into_iter()
            .map(Some)
            .collect::<Vec<Option<Message>>>(),
    );
    let mut messages_prev_vec = messages_vec.clone();
    let mut messages_next_vec = messages_vec.clone();
    messages_prev_vec.push_front(None);
    messages_prev_vec.pop_back();
    messages_next_vec.pop_front();
    messages_next_vec.push_back(None);
    let messages_iter = messages_vec
        .iter()
        .zip(messages_prev_vec)
        .zip(messages_next_vec)
        .map(|((m, p), n)| (rg.clone(), m, p, n))
        .rev();

    cx.render(rsx! {
        div {
            class: "messages",
            messages_iter.map(|(mut rg, message, prev_message, next_message)| {
                if let Some(message) = message {
                    let message_id = message.id();
                    let conversation_id = message.conversation_id();
                    let msg_sender = message.sender().to_string();
                    let replied =  message.replied();
                    let i = ident.did_key().to_string();
                    let remote = i != msg_sender;
                    let first = match prev_message.clone() {
                        Some(prev_message) => prev_message.sender().to_string() != msg_sender,
                        None => true,
                    };
                    let middle = match prev_message {
                        Some(prev_message) => prev_message.sender().to_string() == msg_sender,
                        None => false,
                    };
                    let last = match next_message {
                        Some(next_message) => next_message.sender().to_string() != msg_sender,
                        None => true,
                    };

                    rsx!{
                        Msg {
                            // key: "{message_id}", // todo: try uuid.simple() - it may be that non alpha-numeric characters caused this to panic.
                            message: message.clone(),
                            remote: remote,
                            last: last,
                            first: first,
                            middle: middle,
                            on_reply: move |reply| {
                                if let Err(_e) = warp::async_block_in_place_uncheck(rg.reply(conversation_id, message_id, vec![reply])) {
                                    //TODO: Display error? 
                                }
                            }
                        }
                        match replied {
                            Some(replied) => {
                                let r = cx.props.messaging.clone();
                                match warp::async_block_in_place_uncheck(r.get_message(conversation_id, replied)) {
                                    Ok(message) => {
                                        let lines = message.value().join("\n");
                                        rsx!{
                                            Reply {
                                                message: lines,
                                                is_remote: remote
                                            }
                                        }
                                    },
                                    Err(_) => { rsx!{ span { "Something went wrong" } } }
                                }
                            },
                            _ => rsx!{ div {} }
                        }
                    }
                } else {
                    rsx! { div {} }
                }
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
    })
}
