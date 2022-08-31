use serde::{Deserialize, Serialize};
use warp::crypto::DID;

use self::mutations::Mutations;

pub mod mutations;

const STORAGE_LOCATION: &str = "./.cache/.warpgui.state.json";

pub struct Conversation {
    pub id: DID,
    pub recipients: [DID; 2],
}

pub enum Actions {
    ChatWith(DID),
}

#[derive(Serialize, Deserialize, Default)]
pub struct PersistedState {
    pub chat: Option<DID>,
    pub chats: Vec<DID>,
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
            Actions::ChatWith(did) => Mutations::chat_with(self, did),
        };
        PersistedState {
            chats: self.chats.clone(),
            chat: self.chat.clone(),
        }
    }
}
