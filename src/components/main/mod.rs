use std::time::Duration;

use crate::{components::main::compose::Compose, main::sidebar::Sidebar, RAYGUN, STATE};
use dioxus::prelude::*;
use warp::raygun::Conversation;
use fermi::prelude::*;
pub mod compose;
pub mod sidebar;

#[allow(non_snake_case)]
pub fn Main(cx: Scope) -> Element {
    let state = use_atom_ref(&cx, STATE);

    // TODO:: We should probably write some kind of watcher here for new messages on all conversations
    // so we can create notifications
    // Load Multipass & Raygun's Atom Ref
    let raygun = use_atom_ref(&cx, RAYGUN);

    // Read their values from locks
    let rg = raygun.read().clone().unwrap().clone();

    let st = state.clone();
    cx.spawn(async move {
        loop {
            if let Ok(list) = rg.read().list_conversations().await {
                if !list.is_empty() && st.read().chats != list {
                    st.write().chats = list;
                }
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    });

    let conversation = match state.read().chat.clone() {
        Some(c) => c,
        None => Conversation::default(),
    };

    cx.render(rsx! {
        div {
            class: "main",
            Sidebar {},
            Compose {
                conversation: conversation
            },
        }
    })
}
