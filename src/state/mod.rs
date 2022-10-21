use serde::{Deserialize, Serialize};
use warp::raygun::Conversation;

use crate::DEFAULT_PATH;

use self::mutations::Mutations;

pub mod mutations;

pub enum Actions {
    ChatWith(Conversation),
}

#[derive(Serialize, Deserialize, Default)]
pub struct PersistedState {
    pub chat: Option<Conversation>,
    pub chats: Vec<Conversation>,
}

impl PersistedState {
    pub fn load_or_inital() -> Self {
        match std::fs::read(DEFAULT_PATH.read().join(".uplink.state.json")) {
            Ok(b) => serde_json::from_slice::<PersistedState>(&b).unwrap_or_default(),
            Err(_) => Default::default(),
        }
    }

    pub fn save(&self) {
        if let Ok(bytes) = serde_json::to_vec(self) {
            if let Err(_e) = std::fs::write(DEFAULT_PATH.read().join(".uplink.state.json"), &bytes)
            {
            }
        }
    }

    pub fn dispatch(&mut self, action: Actions) -> Self {
        match action {
            Actions::ChatWith(conversation) => Mutations::chat_with(self, conversation),
        };
        PersistedState {
            chats: self.chats.clone(),
            chat: self.chat.clone(),
        }
    }
}
