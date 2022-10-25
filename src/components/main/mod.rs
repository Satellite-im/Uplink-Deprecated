use crate::{
    main::{compose::Compose, sidebar::Sidebar},
    state::ConversationInfo,
    Account, Messaging, CONVERSATIONS, CONVERSATION_METADATA,
};
use dioxus::prelude::*;
use futures::{stream::FuturesUnordered, StreamExt};
use std::time::Duration;
use uuid::Uuid;
use warp::raygun::{MessageEventKind, MessageOptions, RayGun, RayGunStream};

pub mod compose;
pub mod files;
pub mod friends;
pub mod profile;
pub mod settings;
pub mod sidebar;

#[derive(Props, PartialEq)]
pub struct Prop {
    account: Account,
    messaging: Messaging,
}
enum StreamCmd {
    Increment(Uuid),
}

#[allow(non_snake_case)]
pub fn Main(cx: Scope<Prop>) -> Element {
    let mut rg = cx.props.messaging.clone();
    let chats = use_atom_ref(&cx, CONVERSATIONS).clone();
    let chat_meta = use_atom_ref(&cx, CONVERSATION_METADATA).clone();
    let chat_meta2 = use_atom_ref(&cx, CONVERSATION_METADATA).clone();

    // updates the chat metadata in response to commands sent by the use_future
    let meta_updater = use_coroutine(&cx, |mut rx: UnboundedReceiver<StreamCmd>| async move {
        while let Some(msg) = rx.next().await {
            match msg {
                StreamCmd::Increment(id) => {
                    // assuming this doesn't cause a race condition because the global state is updated inside of use_coroutine
                    if let Some(s) = chat_meta2.write().v.get_mut(&id) {
                        s.total_messages += 1;
                    }
                }
            }
        }
    });
    let meta_updater = meta_updater.clone();

    // assuming this reloads when CONVERSATIONS changes
    use_future(&cx, (), |_| async move {
        // hopefully this allows for polling multiple futures concurrently and cleaning them
        // up automatically on drop
        let mut futures = FuturesUnordered::new();

        // get all conversations
        let conversations = loop {
            if let Ok(list) = rg.list_conversations().await {
                break list;
            }
            // todo: possibly add a loading dialog?
            tokio::time::sleep(Duration::from_millis(300)).await;
        };

        for id in conversations.iter().map(|c| c.id()) {
            // don't open a stream for the current chat
            if let Some(current_chat) = chats.read().current_chat.as_ref() {
                if current_chat.id() == id {
                    continue;
                }
            }

            // update count for messages possibly received while offline
            let messages = rg
                .get_messages(id, MessageOptions::default())
                .await
                .unwrap_or_default();

            if let Some(c) = chat_meta.read().v.get(&id) {
                if messages.len() != c.total_messages {
                    if let Some(s) = chat_meta.write().v.get_mut(&id) {
                        s.total_messages = messages.len();
                    }
                }
            };

            // insert count for new conversation
            if !chat_meta.read().v.contains_key(&id) {
                let m = ConversationInfo {
                    total_messages: messages.len(),
                    ..Default::default() // so cool that you can do this on a struct
                };

                chat_meta.write().v.insert(id, m);
            }

            let mut stream = loop {
                match rg.get_conversation_stream(id).await {
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

            let mu = meta_updater.clone();
            futures.push(async move {
                while let Some(event) = stream.next().await {
                    if let MessageEventKind::MessageReceived {
                        conversation_id, ..
                    } = event
                    {
                        mu.send(StreamCmd::Increment(conversation_id));
                    }
                }
            });
        }

        while futures.next().await.is_some() {
            // should never reach here because the futures don't return
        }
    });

    cx.render(rsx! {
        div {
            class: "main",
            Sidebar {
                messaging: cx.props.messaging.clone(),
                account: cx.props.account.clone(),
            },
            Compose {
                account: cx.props.account.clone(),
                messaging: cx.props.messaging.clone(),
            },
        }
    })
}
