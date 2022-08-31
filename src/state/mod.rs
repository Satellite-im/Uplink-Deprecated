use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::crypto::DID;

use self::mutations::Mutations;

pub mod mutations;

const STORAGE_LOCATION: &str = "./.cache/.warpgui.state.json";

#[derive(Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Conversation {
    pub id: Uuid,
    pub recipients: [DID; 2],
}

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
        match std::fs::read(STORAGE_LOCATION) {
            Ok(b) => serde_json::from_slice::<PersistedState>(&b).unwrap_or_default(),
            Err(_) => Default::default(),
        }
    }

    pub fn save(&self) {
        let bytes = serde_json::to_vec(self.into());
        match bytes {
            Ok(b) => {
                let _ = std::fs::write(STORAGE_LOCATION, &b);
            }
            Err(_) => {}
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
