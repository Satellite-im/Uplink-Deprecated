use crate::{
    main::{compose::Compose, sidebar::Sidebar, welcome::Welcome},
    state::Actions,
    Account, Messaging, STATE,
};

use dioxus::prelude::*;
use futures::StreamExt;
use std::collections::HashMap;
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
    let rg = cx.props.messaging.clone();
    let display_welcome = state.read().selected_chat.is_none();
    let sidebar_visibility = match state.read().hide_sidebar {
        false => "main-sidebar",
        true => "main-chat",
    };

    use_future(&cx, &rg, |mut rg| async move {
        log::debug!("streaming conversations");

        // todo: only accept incoming conversations from people we are friends with.

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

        // get all conversations and update state
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

        // detect removed conversations
        for id in state.read().all_chats.keys() {
            if !conversations.contains_key(id) {
                log::debug!("removing chat");
                state.write().dispatch(Actions::RemoveConversation(*id));
            }
        }

        // detect added conversations
        for (id, conv) in conversations {
            if !state.read().all_chats.contains_key(&id) {
                log::debug!("adding chat");
                state
                    .write()
                    .dispatch(Actions::AddConversation(conv.clone()));
            }
        }

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

                    if !state.read().all_chats.contains_key(&conversation_id) {
                        log::debug!("adding chat");
                        state
                            .write()
                            .dispatch(Actions::AddConversation(conversation));
                    }
                }
                RayGunEventKind::ConversationDeleted { conversation_id } => {
                    if state.read().all_chats.contains_key(&conversation_id) {
                        state
                            .write()
                            .dispatch(Actions::RemoveConversation(conversation_id));
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
