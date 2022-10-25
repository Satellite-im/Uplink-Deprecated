use serde::{Deserialize, Serialize};
use uuid::Uuid;
use warp::raygun::Conversation;

use crate::DEFAULT_PATH;

use self::mutations::Mutations;

pub mod mutations;

pub enum Actions {
    ChatWith(ConversationInfo),
    ConversationsUpdated(Vec<ConversationInfo>),
    UpdateConversation(ConversationInfo),
}

/// tracks the active conversations. Chagnes are persisted
#[derive(Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct Conversations {
    /// the currently selected conversation
    pub current_chat: Option<Uuid>,
    /// all active conversations
    pub all_chats: Vec<ConversationInfo>,
}

/// composes `Conversation` with relevant metadata
#[derive(Serialize, Deserialize, Default, Clone, Eq, PartialEq)]
pub struct ConversationInfo {
    pub conversation: Conversation,
    /// the uuid of the last message read. \
    /// used to determine the number of unread messages
    pub last_msg_read: Option<Uuid>,
    /// the uuid of the last message sent
    pub last_msg_sent: Option<Uuid>,
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
            Actions::ChatWith(info) => self.current_chat = Some(info.conversation.id()),
            Actions::ConversationsUpdated(conversations) => self.all_chats = conversations,
            Actions::UpdateConversation(info) => {
                let new_chats: Vec<ConversationInfo> = self
                    .all_chats
                    .iter()
                    .map(|x| {
                        if x.conversation.id() == info.conversation.id() {
                            info.clone()
                        } else {
                            x.clone()
                        }
                    })
                    .collect();
                self.all_chats = new_chats;
            }
        };
        Conversations {
            all_chats: self.all_chats.clone(),
            current_chat: self.current_chat,
        }
    }
}
