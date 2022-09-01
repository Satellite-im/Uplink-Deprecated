use crate::{components::main::compose::Compose, main::sidebar::Sidebar, STATE, RAYGUN};
use dioxus::prelude::*;
use sir::global_css;
use warp::raygun::Conversation;

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
        if let Ok(list) = rg.write().list_conversations().await {
            if !list.is_empty() && st.read().chats != list {
                st.write().chats = list;
            }
        }
    });

    let conversation = match state.read().chat.clone() {
        Some(c) => c,
        None => Conversation::default(),
    };

    // Start UI
    global_css! {"
    .main {
        display: flex;
        text-align: center;
        width: 100%;
        height: 100%;
        flex-direction: row;
    }
    "}

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
