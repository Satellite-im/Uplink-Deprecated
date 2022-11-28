use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{
    components::main::compose::{msg::Msg, reply::Reply},
    iutils,
    state::{Actions, LastMsgSent},
    Account, Messaging, STATE,
};
use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};

use futures::StreamExt;
use uuid::Uuid;
use warp::{
    crypto::DID,
    raygun::{Message, MessageEvent, MessageEventKind, MessageOptions, RayGun, RayGunStream},
};

#[derive(Eq, PartialEq)]
enum TypingIndicator {
    Typing,
    NotTyping,
}

enum ChanCmd {
    Indicator {
        users_typing: UseRef<HashMap<DID, String>>,
        current_chat: Option<Uuid>,
        remote_id: DID,
        remote_name: String,
        indicator: TypingIndicator,
    },
    Timeout {
        users_typing: UseRef<HashMap<DID, String>>,
        current_chat: Option<Uuid>,
    },
}

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
    users_typing: UseRef<HashMap<DID, String>>,
}

#[allow(non_snake_case)]
pub fn Messages(cx: Scope<Props>) -> Element {
    log::debug!("rendering Messages");

    //Note: We will just unwrap for now though we need to
    //      handle the error properly if there is ever one when
    //      getting own identity
    let state = use_atom_ref(&cx, STATE).clone();

    let mut rg = cx.props.messaging.clone();
    let ident = cx.props.account.read().get_own_identity().unwrap();
    let my_did = ident.did_key().clone();
    // this one has a special name because of the other variable names within the use_future
    let list: UseRef<Vec<Message>> = use_ref(&cx, Vec::new).clone();
    // this one is for the rsx! macro. it is reversed for display purposes and defined here because `list` gets moved into the use_future
    let messages: Vec<Message> = list.read().iter().rev().cloned().collect();

    // this is used for reading the event stream.
    let current_chat = state
        .read()
        .current_chat
        .and_then(|x| state.read().all_chats.get(&x).cloned());

    // periodically refresh message timestamps
    use_future(&cx, (), move |_| {
        let update = cx.schedule_update();
        async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                update();
            }
        }
    });

    // keep track of who is typing by receiving events and adding timeouts
    let chan = use_coroutine(&cx, |mut rx: UnboundedReceiver<ChanCmd>| async move {
        // used for timeouts
        let mut typing_times: HashMap<DID, Instant> = HashMap::new();
        let mut prev_current_chat: Option<Uuid> = None;

        while let Some(cmd) = rx.next().await {
            match cmd {
                ChanCmd::Indicator {
                    users_typing,
                    current_chat,
                    remote_id,
                    remote_name,
                    indicator,
                } => {
                    log::debug!("received typing indicator");
                    if current_chat != prev_current_chat {
                        typing_times.clear();
                        prev_current_chat = current_chat;
                    }
                    if current_chat.is_some() {
                        match indicator {
                            TypingIndicator::Typing => {
                                typing_times.insert(remote_id.clone(), Instant::now());
                                if !users_typing.read().contains_key(&remote_id) {
                                    users_typing.write().insert(remote_id, remote_name);
                                }
                            }
                            TypingIndicator::NotTyping => {
                                typing_times.remove(&remote_id);
                                if users_typing.read().contains_key(&remote_id) {
                                    let _ = users_typing.write().remove(&remote_id);
                                }
                            }
                        }
                    }
                }
                ChanCmd::Timeout {
                    users_typing,
                    current_chat,
                } => {
                    log::debug!("received typing indicator timeout");
                    if current_chat != prev_current_chat {
                        typing_times.clear();
                        prev_current_chat = current_chat;
                    }
                    if current_chat.is_some() {
                        let expired_indicators: HashMap<DID, Instant> = typing_times
                            .iter()
                            .filter(|(_k, v)| {
                                let elapsed = Instant::now().duration_since(**v);
                                elapsed > Duration::from_secs(3)
                            })
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect();

                        let new_users_typing: HashMap<DID, String> = users_typing
                            .read()
                            .iter()
                            .filter(|(k, _v)| !expired_indicators.contains_key(k))
                            .map(|(k, v)| (k.clone(), v.clone()))
                            .collect();

                        for (k, _v) in expired_indicators {
                            let _ = typing_times.remove(&k);
                        }

                        if new_users_typing != *users_typing.read() {
                            *users_typing.write() = new_users_typing;
                        }
                    }
                }
            }
        }
    });

    // periodically check for timeouts
    let chan1 = chan.clone();
    let real_current_chat = state.read().current_chat.clone();
    use_future(
        &cx,
        (&real_current_chat.clone(), &cx.props.users_typing.clone()),
        |(current_chat, users_typing)| async move {
            loop {
                log::debug!("checking for typing indicator timeout on rx side");
                tokio::time::sleep(Duration::from_secs(4)).await;
                chan1.send(ChanCmd::Timeout {
                    users_typing: users_typing.clone(),
                    current_chat,
                });
            }
        },
    );

    // handle message stream
    let chan2 = chan.clone();
    use_future(
        &cx,
        (
            &current_chat,
            &cx.props.users_typing.clone(),
            &cx.props.account.clone(),
        ),
        |(current_chat, users_typing, mp)| async move {
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
                log::debug!("updating messages list ");
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
                                    log::debug!("compose/messages streamed a new message ");
                                    // remove typing indicator
                                    let username =
                                        iutils::get_username_from_did(message.sender(), &mp);
                                    chan2.send(ChanCmd::Indicator {
                                        users_typing: users_typing.clone(),
                                        current_chat: Some(conversation_id),
                                        remote_id: message.sender(),
                                        remote_name: username,
                                        indicator: TypingIndicator::NotTyping,
                                    });
                                    // update messages
                                    // todo: check if sidebar gets updated
                                    list.write().push(message.clone());
                                    current_chat.last_msg_sent =
                                        Some(LastMsgSent::new(&message.value()));
                                    state.write().dispatch(Actions::UpdateConversation(
                                        current_chat.clone(),
                                    ));
                                }
                                Err(_e) => {
                                    // todo: log error
                                }
                            }
                        }
                    }
                    MessageEventKind::EventReceived {
                        conversation_id,
                        did_key,
                        event,
                    } => match event {
                        MessageEvent::Typing => {
                            if current_chat.conversation.id() == conversation_id
                                && did_key != my_did
                            {
                                let username = iutils::get_username_from_did(did_key.clone(), &mp);
                                chan2.send(ChanCmd::Indicator {
                                    users_typing: users_typing.clone(),
                                    current_chat: Some(conversation_id),
                                    remote_id: did_key,
                                    remote_name: username,
                                    indicator: TypingIndicator::Typing,
                                })
                            }
                        }
                    },
                    MessageEventKind::EventCancelled {
                        conversation_id,
                        did_key,
                        event,
                    } => match event {
                        // this event isn't expected to be sent. handling it here anyway.
                        MessageEvent::Typing => {
                            if current_chat.conversation.id() == conversation_id
                                && did_key != my_did
                            {
                                let username = iutils::get_username_from_did(did_key.clone(), &mp);
                                chan2.send(ChanCmd::Indicator {
                                    users_typing: users_typing.clone(),
                                    current_chat: Some(conversation_id),
                                    remote_id: did_key,
                                    remote_name: username,
                                    indicator: TypingIndicator::NotTyping,
                                })
                            }
                        }
                    },
                    _ => {}
                }
            }
        },
    );

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
                .map(|(mut rg, message, next_sender, prev_sender)| {
                    let message_id = message.id();
                    let conversation_id = message.conversation_id();
                    let msg_sender = message.sender();
                    let is_remote = ident.did_key() != msg_sender;
                    let is_last = next_sender.map(|next_sender| *next_sender != msg_sender).unwrap_or(true);
                    let is_first = prev_sender.map(|prev_sender| *prev_sender != msg_sender).unwrap_or(true);

                    rsx! {
                        div {
                            key: "{message_id}",
                            style: "display: contents",
                            Msg {
                                // key: "{message_id}-reply",
                                messaging: cx.props.messaging.clone(),message: message.clone(),
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
                                }
                            }
                            match message.replied() {
                                Some(replied) => {
                                    let r = cx.props.messaging.clone();
                                    match warp::async_block_in_place_uncheck(r.get_message(conversation_id, replied)) {
                                        Ok(message) => {
                                            rsx!{
                                                Reply {
                                                    // key: "{message_id}-reply",
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
                                _ => rsx!{ div {  } }
                            }
                        }
                    }
                }),
            div {
                // key: "encrypted-notification-0001",
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
