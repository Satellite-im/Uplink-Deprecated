use crate::{
    main::{compose::Compose, sidebar::Sidebar, welcome::Welcome},
    state::{Actions, ConversationInfo},
    Account, Messaging, STATE,
};
use chrono::prelude::*;
use dioxus::prelude::*;
use std::{collections::HashMap, time::Duration};
use uuid::Uuid;
use warp::raygun::RayGun;

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
        loop {
            if let Ok(list) = rg.list_conversations().await {
                let mut current_conversations: HashMap<Uuid, ConversationInfo> = HashMap::new();
                for item in &list {
                    let to_insert = match st.read().all_chats.get(&item.id()) {
                        Some(v) => v.clone(),
                        None => ConversationInfo {
                            conversation: item.clone(),
                            creation_time: DateTime::from(Local::now()),
                            ..Default::default()
                        },
                    };
                    current_conversations.insert(item.id(), to_insert);
                }
                if current_conversations != st.read().all_chats {
                    log::debug!("modifying chats");
                    st.write()
                        .dispatch(Actions::AddRemoveConversations(current_conversations));
                }
            }
            // TODO: find a way to sync this with the frame rate or create a "polling rate" value we can configure
            // This also doesn't really seem to effect performance
            tokio::time::sleep(Duration::from_secs(1)).await;
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
