use crate::{
    components::main::compose::{msg::Msg, reply::Reply},
    state::{Actions, LastMsgSent},
    Account, Messaging, STATE,
};
use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};

use futures::StreamExt;
use warp::{
    crypto::DID,
    raygun::{Message, MessageEventKind, MessageOptions, RayGun, RayGunStream},
};

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
    // this one is for the rsx! macro. it is reversed for display purposes and defined here because `list` gets moved into the use_future
    let messages: Vec<Message> = list.read().iter().rev().cloned().collect();

    let current_chat = state
        .read()
        .current_chat
        .and_then(|x| state.read().all_chats.get(&x).cloned());

    // periodically refresh message timestamps
    let should_reload = use_future(&cx, (), |_| async move {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        true
    });

    if should_reload.value().is_some() {
        cx.needs_update();
    };

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
                        match rg.get_message(conversation_id, message_id).await {
                            Ok(message) => {
                                list.write().push(message.clone());
                                // todo: add message to chats sidebar
                                current_chat.last_msg_sent =
                                    Some(LastMsgSent::new(&message.value()));
                                state
                                    .write()
                                    .dispatch(Actions::UpdateConversation(current_chat.clone()));
                            }
                            Err(_e) => {
                                // todo: log error
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    });

    let rg = cx.props.messaging.clone();
    let senders: Vec<DID> = messages.iter().map(|msg| msg.sender()).collect();
    // messages has already been reversed
    let idx_range = 0..messages.len();
    let next_sender = idx_range.clone().map(|idx| senders.get(idx + 1));
    let prev_sender = idx_range.map(|idx| if idx == 0 { None } else { senders.get(idx - 1) });

    cx.render(rsx! {
        div {
            class: "messages",
            messages.iter()
                .zip(next_sender)
                .zip(prev_sender)
                .map(|((message, next_sender), prev_sender)| (rg.clone(), message, next_sender, prev_sender))
            .map(|(mut rg, message, next_sender, prev_sender)|{
                let message_id = message.id();
                let conversation_id = message.conversation_id();
                let msg_sender = message.sender();
                let is_remote = ident.did_key() != msg_sender;
                let is_last = next_sender.map(|next_sender| *next_sender != msg_sender).unwrap_or(true);
                let is_first = prev_sender.map(|prev_sender| *prev_sender != msg_sender).unwrap_or(true);

                rsx!{
                    Msg {
                        // key: "{message_id}", // todo: try uuid.simple() - it may be that non alpha-numeric characters caused this to panic.
                        message: message.clone(),
                        account: cx.props.account.clone(),
                        sender: message.sender(),
                        remote: is_remote,
                        // not sure why this works. I believe the calculations for is_last and is_first are correct but for an unknown reason the time and profile picture gets displayed backwards.
                        last:  is_first,
                        first: is_last,
                        middle: !is_last && !is_first,
                        on_reply: move |reply| {
                            if let Err(_e) = warp::async_block_in_place_uncheck(rg.reply(conversation_id, message_id, vec![reply])) {
                                //TODO: Display error?
                            }
                        },
                        on_typing_reply: move |typing| {
                            if let Err(_e) = warp::async_block_in_place_uncheck(rg.trigger_typing(conversation_id, typing)) {
                            }
                    },
                    match message.replied() {
                        Some(replied) => {
                            let r = cx.props.messaging.clone();
                            match warp::async_block_in_place_uncheck(r.get_message(conversation_id, replied)) {
                                Ok(message) => {
                                    rsx!{
                                        Reply {
                                            message: message.value().join("\n"),
                                            is_remote: is_remote,
                                            account: cx.props.account.clone(),
                                            sender: message.sender(),
                                        }
                                    }
                                },
                                Err(_) => { rsx!{ span { "Something went wrong" } } }
                            }
                        },
                        _ => rsx!{ div {} }
                    }
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
