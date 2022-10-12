pub mod messages;
pub mod msg;
pub mod topbar;
pub mod write;

use dioxus::prelude::*;
use dioxus_heroicons::outline::Shape;
use futures::StreamExt;
use warp::raygun::{Conversation, MessageEventKind, MessageOptions, RayGun, RayGunStream};

use crate::{
    components::{
        main::compose::{messages::Messages, topbar::TopBar, write::Write},
        ui_kit::button::Button,
    },
    Account, Messaging, LANGUAGE, STATE,
};

#[derive(PartialEq, Props)]
pub struct Props {
    account: Account,
    messaging: Messaging,
    conversation: Conversation,
}

#[allow(non_snake_case)]
pub fn Compose(cx: Scope<Props>) -> Element {
    let state = use_atom_ref(&cx, STATE);
    let ext_conversation_id = cx.props.conversation.id();
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let warningMessage = l.prerelease_warning.to_string();

    let blur = state.read().chat.is_none();
    let text = use_state(&cx, || String::from(""));
    let show_warning = use_state(&cx, || true);
    let rg = cx.props.messaging.clone();

    let messages = use_state(&cx, Vec::new);

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
        if *list.get() != messages {
            list.set(
                rg.get_messages(ext_conversation_id, MessageOptions::default())
                    .await
                    .unwrap_or_default(),
            );
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
                            list.with_mut(|l| l.push(message));
                        }
                    }
                }
                _ => {}
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "compose",
            if blur {
                rsx!(
                    div {
                        class: "blurmask"
                    }
                )
            } else {
                rsx!(
                    TopBar {
                        account: cx.props.account.clone(),
                        conversation: cx.props.conversation.clone(),
                        on_call: move |_| {},
                    }
                )
            },
            (**show_warning).then(|| rsx!(
                div {
                    class: "alpha-warning animate__animated animate__slideInDown",
                    "{warningMessage}",
                    Button {
                        on_pressed: move |_| {
                            show_warning.set(false);
                        },
                        icon: Shape::Check,
                        text: l.user_agrees.to_string(),
                    }
                },
            ))
            div {
                class: "messages-container",
                Messages {
                    account: cx.props.account.clone(),
                    messages: messages.to_vec(),
                }
            },
            div {
                class: "writer-container",
                Write {
                    on_submit: move |message: String| {
                        text.set(String::from(""));
                        let mut rg = cx.props.messaging.clone();

                        let text_as_vec = message
                            .split('\n')
                            .filter(|&s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect::<Vec<_>>();

                        // TODO: We need to wire this message up to display differently
                        // until we confim whether it was successfully sent or failed
                        match warp::async_block_in_place_uncheck(rg.send(ext_conversation_id, None, text_as_vec)) {
                            Ok(_) => {},
                            Err(_e) => {
                                //TODO: Handle error?
                            }
                        };
                    },
                    on_upload: move |_| {}
                }
            }
        }
    })
}
