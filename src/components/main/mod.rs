use crate::{
    main::{compose::Compose, sidebar::Sidebar, welcome::Welcome},
    state::{Actions, ConversationInfo},
    Account, Messaging, STATE,
};
use dioxus::prelude::*;
use std::{collections::HashMap, time::Duration};
use uuid::Uuid;
use warp::raygun::{Conversation, RayGun};

pub mod compose;
pub mod files;
pub mod friends;
pub mod profile;
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
    let st = use_atom_ref(&cx, STATE).clone();
    let rg = cx.props.messaging.clone();
    let state = use_atom_ref(&cx, STATE);
    let display_welcome = state.read().current_chat.is_none();

    use_future(&cx, (), |_| async move {
        loop {
            if let Ok(list) = rg.list_conversations().await {
                let mut incoming: HashMap<Uuid, Conversation> = HashMap::new();
                for item in &list {
                    incoming.insert(item.id(), item.clone());
                }
                let new_chats: Vec<&Conversation> = list
                    .iter()
                    .filter(|x| !st.read().all_chats.contains_key(&x.id()))
                    .collect();
                let to_remove: Vec<Uuid> = st
                    .read()
                    .all_chats
                    .iter()
                    .filter(|(uuid, _)| !incoming.contains_key(uuid))
                    .map(|(uuid, _)| *uuid)
                    .collect();
                if !new_chats.is_empty() || !to_remove.is_empty() {
                    let mut new_map = st.read().all_chats.clone();
                    for id in to_remove {
                        new_map.remove(&id);
                    }
                    for item in new_chats {
                        let ci = ConversationInfo {
                            conversation: item.clone(),
                            ..Default::default()
                        };
                        new_map.insert(item.id(), ci);
                    }

                    st.write()
                        .dispatch(Actions::AddRemoveConversations(new_map));
                }
            }
            // TODO: find a way to sync this with the frame rate or create a "polling rate" value we can configure
            // This also doesn't really seem to effect performance
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    cx.render(rsx! {
        div {
            class: "main",
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
