use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use warp::raygun::Conversation;

use crate::DEFAULT_PATH;

use self::mutations::Mutations;

pub mod mutations;

pub enum Actions {
    ChatWith(Conversation),
}

/// tracks the active conversations. Chagnes are persisted
#[derive(Serialize, Deserialize, Default)]
pub struct Conversations {
    /// the currently selected conversation
    pub current_chat: Option<Conversation>,
    /// all active conversations
    pub all_chats: Vec<Conversation>,
}

/// used to display information associated with each chat
/// belongs in a HashMap<Uuid, ChatInfo>
/// gets saved to pocket_dimension
#[derive(Serialize, Deserialize, Default)]
pub struct ConversationInfo {
    /// the total messages that have been recevied for this chat
    pub total_messages: u32,
    /// the value of total_messages last time the chat was read
    pub last_read: u32,
    /// the uuid of the last message sent
    pub last_msg: Uuid,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ConversationMetadata {
    // `v` stands for value. wrapped this HashMap in a struct to facilitate persistent storage
    /// key: conversation id
    pub v: HashMap<Uuid, ConversationInfo>,
}

impl Conversations {
    pub fn load_or_inital() -> Self {
        match std::fs::read(DEFAULT_PATH.read().join(".uplink.conversations.json")) {
            Ok(b) => serde_json::from_slice::<Conversations>(&b).unwrap_or_default(),
            Err(_) => Default::default(),
        }
    }

    pub fn save(&self) {
        if let Ok(bytes) = serde_json::to_vec(self) {
            if let Err(_e) = std::fs::write(
                DEFAULT_PATH.read().join(".uplink.conversations.json"),
                &bytes,
            ) {}
        }
    }

    pub fn dispatch(&mut self, action: Actions) -> Self {
        match action {
            Actions::ChatWith(conversation) => Mutations::chat_with(self, conversation),
        };
        Conversations {
            all_chats: self.all_chats.clone(),
            current_chat: self.current_chat.clone(),
        }
    }
}

impl ConversationMetadata {
    pub fn load_or_inital() -> Self {
        match std::fs::read(
            DEFAULT_PATH
                .read()
                .join(".uplink.conversation_metadata.json"),
        ) {
            Ok(b) => serde_json::from_slice::<ConversationMetadata>(&b).unwrap_or_default(),
            Err(_) => Default::default(),
        }
    }

    pub fn save(&self) {
        if let Ok(bytes) = serde_json::to_vec(self) {
            if let Err(_e) = std::fs::write(
                DEFAULT_PATH
                    .read()
                    .join(".uplink.conversation_metadata.json"),
                &bytes,
            ) {}
        }
    }
}
