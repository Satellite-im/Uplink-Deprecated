use crate::{components::main::compose::msg::Msg, Account, Messaging, STATE};
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
    let state = use_atom_ref(&cx, STATE);
    //Note: We will just unwrap for now though we need to
    //      handle the error properly if there is ever one when
    //      getting own identity
    let ident = cx.props.account.read().get_own_identity().unwrap();
    let messages = use_ref(&cx, Vec::new);
    let ext_conversation_id = state.read().chat.as_ref().map(|conv| conv.id());

    let rg = cx.props.messaging.clone();
    //Note: Broken for the time being as switching conversation doesnt clear out
    //      messages.
    use_future(
        &cx,
        (messages, &rg, &ext_conversation_id),
        |(list, mut rg, input_conversation_id)| async move {
            // don't stream messages from a nonexistent conversation
            let ext_conversation_id = match input_conversation_id {
                Some(id) => id,
                None => return,
            };

            let mut stream = loop {
                match rg.get_conversation_stream(ext_conversation_id).await {
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
        },
    );

    cx.render({
        let mut prev_sender = "".to_string();
        
        rsx! {
            div {
                class: "messages",
                messages.read().iter().rev().map(|message| (rg.clone(), message)).map(|(mut rg, message)|{
                    let message_id = message.id();
                    let conversation_id = message.conversation_id();
                    let msg_sender = message.sender().to_string();
                    let replied =  message.replied();
                    let i = ident.did_key().to_string();
                    let remote = i != msg_sender;
                    let last = prev_sender != msg_sender;
                    let middle = prev_sender == msg_sender;
                    let first = false;

                    prev_sender = message.sender().to_string();
                    
                    rsx!{
                        match replied {
                            Some(replied) => {
                                match warp::async_block_in_place_uncheck(rg.get_message(conversation_id, replied)) {
                                    Ok(message) => {
                                        let lines = message.value().join("\n");
                                        rsx!{
                                            p {
                                                "{lines}"
                                            }
                                        }
                                        
                                    },
                                    Err(_) => {
                                        rsx!{
                                            p {
                                                "Message Dont exist"
                                            }
                                        } 
                                    }
                                }
                            },
                            _ => rsx!{ div {} }
                        }
                        Msg {
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
        }
    })
}
