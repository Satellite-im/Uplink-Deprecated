use crate::{components::main::compose::msg::Msg, Account, Messaging};
use dioxus::prelude::*;
use dioxus_heroicons::{outline::Shape, Icon};
use warp::raygun::{Conversation, MessageOptions, RayGun};

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
    messaging: Messaging,
    conversation: Conversation,
}

#[allow(non_snake_case)]
pub fn Messages(cx: Scope<Props>) -> Element {
    let conversation_id = cx.props.conversation.id();

    let mp = cx.props.account.clone();
    let rg = cx.props.messaging.clone();
    let messages = use_state(&cx, || {
        warp::async_block_in_place_uncheck(
            rg.get_messages(conversation_id, MessageOptions::default()),
        ).unwrap_or_default()
    });
    
    //Set an error if conversation could not be fetched
    //Note: Broken for the time being
    use_future(&cx, (messages, &rg), |(list, rg)| async move {
        loop {
            let rg_list = match rg
                .get_messages(conversation_id, MessageOptions::default())
                .await
            {
                Ok(l) => l,
                Err(_) => break, //TODO: Error?
            };

            if *list.get() != rg_list {
                list.set(rg_list);
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }

        // TODO: Use this instead for handling events
        // let mut stream = match rg.get_conversation_stream(conversation_id).await {
        //     Ok(stream) => stream,
        //     Err(e) => {
        //         return
        //     },
        // };

        // while let Some(event) = stream.next().await {
        //     match event {
        //         MessageEventKind::MessageReceived {
        //             conversation_id,
        //             message_id,
        //         }
        //         | MessageEventKind::MessageSent {
        //             conversation_id,
        //             message_id,
        //         } => {
        //             if let Ok(message) = rg.get_message(conversation_id, message_id).await {
        //                 list.make_mut().push(message);
        //             }
        //         }
        //         _ => {}
        //     }
        // }
    });

    //Note: We will just unwrap for now though we need to
    //      handle the error properly if there is ever one when
    //      getting own identity
    let ident = mp.read().get_own_identity().unwrap();

    cx.render({
            let mut prev_sender = "".to_string();
            rsx! {
                div {
                    class: "messages",
                    messages.iter().rev().map(|message|{
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
                            "Messages are encrypted locally, anyone outside of this chat cannot modify, or read them."
                        }
                    }
                }
            }
    })
}
