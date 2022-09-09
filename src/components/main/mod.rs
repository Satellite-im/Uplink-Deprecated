use std::time::Duration;

use crate::{
    components::main::compose::Compose, main::sidebar::Sidebar, Account, Messaging, state::PersistedState,
};
use dioxus::prelude::*;
use warp::raygun::Conversation;

pub mod compose;
pub mod sidebar;

#[derive(Props, PartialEq)]
pub struct Prop {
    state: PersistedState,
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Main(cx: Scope<Prop>) -> Element {
    // TODO:: We should probably write some kind of watcher here for new messages on all conversations
    // so we can create notifications
    let state = cx.props.state.clone();
    // Load Multipass & Raygun's Atom Ref
    let raygun = cx.props.messaging.clone();

    // Read their values from locks
    let rg = raygun;

    cx.spawn(async move {
        loop {
            if let Ok(list) = rg.read().list_conversations().await {
                if !list.is_empty() && *state.chats.read() != list {
                    *state.chats.write() = list;
                }
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    });

    let conversation = match state.chat.read().clone() {
        Some(c) => c,
        None => Conversation::default(),
    };

    cx.render(rsx! {
        div {
            class: "main",
            Sidebar {
                state: cx.props.state.clone(),
                messaging: cx.props.messaging.clone(),
                account: cx.props.account.clone()
            },
            Compose {
                state: cx.props.state.clone(),
                account: cx.props.account.clone(),
                messaging: cx.props.messaging.clone(),
                conversation: conversation
            },
        }
    })
}
