use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use warp::raygun::Conversation;

use crate::DEFAULT_PATH;

pub mod mutations;

pub enum Actions {
    ChatWith(ConversationInfo),
    AddRemoveConversations(HashMap<Uuid, ConversationInfo>),
    UpdateConversation(ConversationInfo),
}

/// tracks the active conversations. Chagnes are persisted
#[derive(Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct PersistedState {
    /// the currently selected conversation
    pub current_chat: Option<Uuid>,
    /// all active conversations
    pub all_chats: HashMap<Uuid, ConversationInfo>,
}

/// composes `Conversation` with relevant metadata
#[derive(Serialize, Deserialize, Default, Clone, Eq, PartialEq)]
pub struct ConversationInfo {
    pub conversation: Conversation,
    /// the uuid of the last message read. \
    /// used to determine the number of unread messages
    pub last_msg_read: Option<Uuid>,
    /// the first two lines of the last message sent
    pub last_msg_sent: Option<Vec<String>>,
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
                eprintln!("error saving: {}", _e);
            }
        }
    }

    pub fn dispatch(&mut self, action: Actions) -> Self {
        match action {
            Actions::ChatWith(info) => self.current_chat = Some(info.conversation.id()),
            Actions::AddRemoveConversations(new_chats) => {
                self.all_chats = new_chats;
            }
            Actions::UpdateConversation(info) => {
                // overwrite the existing entry
                self.all_chats.insert(info.conversation.id(), info);
            }
        };
        PersistedState {
            all_chats: self.all_chats.clone(),
            current_chat: self.current_chat,
        }
    }
}
