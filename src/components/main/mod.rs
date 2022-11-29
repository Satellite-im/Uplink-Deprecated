use crate::{
    main::{compose::Compose, sidebar::Sidebar, welcome::Welcome},
    state::{Actions, ConversationInfo},
    Account, Messaging, STATE,
};
use chrono::prelude::*;
use dioxus::prelude::*;
use futures::StreamExt;
use std::{collections::HashMap, time::Duration};
use uuid::Uuid;
use warp::raygun::RayGunEventKind;

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
    let st = use_atom_ref(&cx, STATE).clone();
    let rg = cx.props.messaging.clone();
    let state = use_atom_ref(&cx, STATE);
    let display_welcome = state.read().current_chat.is_none();
    let sidebar_visibility = match st.read().hide_sidebar {
        false => "main-sidebar",
        true => "main-chat",
    };
    use_future(&cx, (), |_| async move {
        let mut rg = rg.clone();

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

        if let Ok(list) = rg.list_conversations().await {
            for item in &list {
                if !st.read().all_chats.contains_key(&item.id()) {
                    log::debug!("modifying chats");
                    st.write().dispatch(Actions::AddConversation(item.clone()));
                };
            }
        }

        while let Some(event) = stream.next().await {
            match event {
                RayGunEventKind::ConversationCreated { conversation_id } => {
                    //For now get the conversation from list_conversation
                    if let Some(conversation) = rg
                        .list_conversations()
                        .await
                        .unwrap_or_default()
                        .iter()
                        .filter(|convo| convo.id() == conversation_id)
                        .cloned()
                        .collect::<Vec<_>>()
                        .first()
                        .cloned()
                    {
                        st.write().dispatch(Actions::AddConversation(conversation));
                    }
                }
                RayGunEventKind::ConversationDeleted { .. } => {
                    //TODO
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
