use crate::{components::main::compose::msg::Msg, Account, Messaging};
use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};

use futures::StreamExt;
use warp::raygun::{Conversation, MessageEventKind, MessageOptions, RayGun, RayGunStream};

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
    conversation: Conversation,
}

#[allow(non_snake_case)]
pub fn Messages(cx: Scope<Props>) -> Element {
    //Note: We will just unwrap for now though we need to
    //      handle the error properly if there is ever one when
    //      getting own identity
    let ident = cx.props.account.read().get_own_identity().unwrap();
    let messages = use_ref(&cx, Vec::new);
    let ext_conversation_id = cx.props.conversation.id();

    let rg = cx.props.messaging.clone();

    //Note: Broken for the time being as switching conversation doesnt clear out
    //      messages.
    use_future(&cx, (messages, &rg), |(list, mut rg)| async move {
        // loop {
        //     let rg_list = match rg
        //         .get_messages(conversation_id, MessageOptions::default())
        //         .await
        //     {
        //         Ok(l) => l,
        //         Err(warp::error::Error::RayGunExtensionUnavailable) => continue,
        //         Err(_e) => {
        //             //Do we want to break this loop?
        //             break;
        //         }
        //     };

        //     if *list.get() != rg_list {
        //         list.set(rg_list);
        //     }
        //     tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        // }

        // TODO: Use this instead for handling events

        let mut stream = loop {
            match rg.get_conversation_stream(ext_conversation_id).await {
                Ok(stream) => break stream,
                Err(warp::error::Error::RayGunExtensionUnavailable) => {
                    //Give sometime for everything in the background to fully line up
                    //Note, if this error still happens, it means there is an fatal error
                    //      in the background
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                    continue;
                }
                //TODO: Provide error in some way.
                Err(_e) => return,
            }
        };
        let messages = rg
            .get_messages(ext_conversation_id, MessageOptions::default())
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
                    if ext_conversation_id == conversation_id {
                        if let Ok(message) = rg.get_message(conversation_id, message_id).await {
                            list.write().push(message);
                        }
                    }
                }
                _ => {}
            }
        }
    });

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
