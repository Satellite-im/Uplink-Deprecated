use crate::{
    main::{compose::Compose, sidebar::Sidebar, welcome::Welcome},
    state::{Actions, ConversationInfo},
    Account, Messaging, ALL_CHATS, STATE,
};
use chrono::prelude::*;
use dioxus::prelude::*;
use futures::StreamExt;
use std::{collections::HashMap, time::Duration};
use uuid::Uuid;
use warp::raygun::{Conversation, RayGunEventKind};

pub mod compose;
pub mod files;
pub mod friends;
pub mod settings;
pub mod sidebar;
pub mod welcome;

#[derive(Props, PartialEq)]
pub struct Prop {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Main(cx: Scope<Prop>) -> Element {
    log::debug!("rendering Main");
    let state = use_atom_ref(&cx, STATE).clone();
    let all_chats = use_atom_ref(&cx, ALL_CHATS).clone();
    let rg = cx.props.messaging.clone();
    let display_welcome = state.read().current_chat.is_none();
    let sidebar_visibility = match state.read().hide_sidebar {
        false => "main-sidebar",
        true => "main-chat",
    };

    use_future(&cx, &rg, |mut rg| async move {
        log::debug!("streaming conversations");

        // get all conversations and store them in glboal state

        // get all conversations
        let mut conversations: HashMap<Uuid, Conversation> = HashMap::new();
        match rg.list_conversations().await {
            Ok(r) => {
                for c in r {
                    conversations.insert(c.id(), c.clone());
                }
            }
            Err(e) => {
                log::error!("failed to get conversations: {}", e);
                return;
            }
        }

        // create new Hashmap of all conversations
        let mut new_chats: HashMap<Uuid, Conversation> = HashMap::new();
        for (id, conv) in conversations {
            match all_chats.read().get(&id) {
                Some(c) => new_chats.insert(id, c.clone()),
                None => new_chats.insert(id, conv),
            };
        }

        // update global state
        *all_chats.write() = new_chats;

        // receive events from Warp
        let mut stream = loop {
            match rg.subscribe().await {
                Ok(stream) => break stream,
                Err(warp::error::Error::MultiPassExtensionUnavailable)
                | Err(warp::error::Error::RayGunExtensionUnavailable) => {
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                }
                Err(_e) => {
                    //Should not reach this point but should handle an error if it does
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                }
            }
        };

        while let Some(event) = stream.next().await {
            match event {
                RayGunEventKind::ConversationCreated { conversation_id } => {
                    //For now get the conversation from list_conversation
                    let conversation = rg
                        .list_conversations()
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter(|convo| convo.id() == conversation_id)
                        .cloned()
                        .collect::<Vec<_>>()
                        .first()
                        .cloned()
                        .unwrap();

                    if !all_chats.read().contains_key(&conversation.id()) {
                        log::debug!("adding new conversation");
                        all_chats.write().insert(conversation.id(), conversation);
                    } else {
                        log::debug!("received ConversationCreated event for existing conversation");
                    }
                }
                RayGunEventKind::ConversationDeleted { conversation_id } => {
                    if all_chats.write().remove(&conversation_id).is_none() {
                        log::debug!("attempted to remove conversation which didn't exist");
                    } else {
                        log::debug!("removed conversation");
                    }
                }
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "main {sidebar_visibility}",
            rsx!(
                Sidebar {
                    messaging: cx.props.messaging.clone(),
                    account: cx.props.account.clone(),
                },
                if display_welcome {
                    rsx!(
                        Welcome {}
                    )
                } else {
                    rsx!(
                        Compose {
                            account: cx.props.account.clone(),
                            messaging: cx.props.messaging.clone(),
                        }
                    )
                }
            )
        }
    })
}
