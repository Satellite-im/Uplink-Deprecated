use serde::{Deserialize, Serialize};
use std::sync::Arc;
use warp::raygun::Conversation;
use warp::sync::RwLock;

use crate::DEFAULT_PATH;

pub mod mutations;

pub enum Actions {
    ChatWith(Conversation),
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PersistedState {
    pub chat: Arc<RwLock<Option<Conversation>>>,
    pub chats: Arc<RwLock<Vec<Conversation>>>,
}

impl PartialEq for PersistedState {
    fn eq(&self, other: &Self) -> bool {
        *self.chat.read() == *other.chat.read() && *self.chats.read() == *other.chats.read()
    }
}

impl PersistedState {
    pub fn load_or_inital() -> Self {
        match std::fs::read(DEFAULT_PATH.read().join(".warpgui.state.json")) {
            Ok(b) => serde_json::from_slice::<PersistedState>(&b).unwrap_or_default(),
            Err(_) => Default::default(),
        }
    }

    pub fn save(&self) {
        if let Ok(bytes) = serde_json::to_vec(self) {
            if let Err(_e) = std::fs::write(DEFAULT_PATH.read().join(".warpgui.state.json"), &bytes)
            {
            }
        }
    }

    pub fn dispatch(&self, action: Actions) -> Self {
        match action {
            Actions::ChatWith(conversation) => {
                for (i, chat) in self.chats.read().clone().iter().enumerate() {
                    let mut recipients_equal = true;
                    for recipient in chat.recipients().clone() {
                        if !conversation.recipients().contains(&recipient) {
                            recipients_equal = false;
                            break;
                        }
                    }

                    if recipients_equal {
                        self.chats.write().remove(i);
                    }
                }
                self.chats.write().push(conversation.clone());
                *self.chat.write() = Some(conversation.clone());
            }
        }

        PersistedState {
            chats: self.chats.clone(),
            chat: self.chat.clone(),
        }
    }
}
