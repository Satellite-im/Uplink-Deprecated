use std::time::Duration;

use crate::{
    components::main::{compose::Compose}, main::sidebar::Sidebar, Account, Messaging, STATE,
};
use dioxus::prelude::*;
use warp::raygun::{Conversation, RayGun};

pub mod compose;
pub mod sidebar;
pub mod friends;
pub mod profile;
pub mod settings;

#[derive(Props, PartialEq)]
pub struct Prop {
    account: Account,
    messaging: Messaging,
}

#[allow(non_snake_case)]
pub fn Main(cx: Scope<Prop>) -> Element {
    let state = use_atom_ref(&cx, STATE);

    // Read their values from locks
    let rg = cx.props.messaging.clone();

    let st = state.clone();
    use_future(&cx, (), |_| async move {
        loop {
            if let Ok(list) = rg.list_conversations().await {
                if !list.is_empty() && st.read().chats != list {
                    st.write().chats = list;
                }
            }
            // TODO: find a way to sync this with the frame rate or create a "polling rate" value we can configure
            // This also doesn't really seem to effect performance
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    let conversation = match state.read().chat.clone() {
        Some(c) => c,
        None => Conversation::default(),
    };

    cx.render(rsx! {
        div {
            class: "main",
            Sidebar {
                messaging: cx.props.messaging.clone(),
                account: cx.props.account.clone()
            },
            Compose {
                account: cx.props.account.clone(),
                messaging: cx.props.messaging.clone(),
                conversation: conversation
            },
        }
    })
}
