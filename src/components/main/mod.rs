use crate::{
    main::{compose::Compose, sidebar::Sidebar},
    state::{Actions, ConversationInfo},
    Account, Messaging, CONVERSATIONS,
};
use dioxus::prelude::*;
use std::{collections::HashSet, time::Duration};
use uuid::Uuid;
use warp::raygun::{Conversation, RayGun};

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

#[allow(non_snake_case)]
pub fn Main(cx: Scope<Prop>) -> Element {
    let st = use_atom_ref(&cx, CONVERSATIONS).clone();
    let rg = cx.props.messaging.clone();

    use_future(&cx, (), |_| async move {
        loop {
            if let Ok(list) = rg.list_conversations().await {
                // not the most efficient
                let mut set: HashSet<Uuid> = HashSet::new();
                st.read()
                    .all_chats
                    .iter()
                    // extract the Conversation from the ConversationInfo
                    .map(|x| x.conversation.clone())
                    // insert it into the set
                    .for_each(|x| {
                        set.insert(x.id());
                    });
                let new_chats: Vec<&Conversation> =
                    list.iter().filter(|x| !set.contains(&x.id())).collect();
                if !new_chats.is_empty() {
                    let mut updated_list = st.read().all_chats.clone();
                    updated_list.extend(new_chats.iter().map(|conversation| ConversationInfo {
                        conversation: (*conversation).clone(),
                        ..Default::default()
                    }));
                    st.write()
                        .dispatch(Actions::ConversationsUpdated(updated_list))
                        .save();
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
