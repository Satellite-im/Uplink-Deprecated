use chrono::prelude::*;
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

#[derive(Serialize, Deserialize, Default, Clone, Eq, PartialEq)]
pub struct LastMsgSent {
    pub value: String,
    pub time: DateTime<Local>,
}

/// composes `Conversation` with relevant metadata
#[derive(Serialize, Deserialize, Default, Clone, Eq, PartialEq)]
pub struct ConversationInfo {
    pub conversation: Conversation,
    /// the uuid of the last message read. \
    /// used to determine the number of unread messages
    pub last_msg_read: Option<Uuid>,
    /// the first two lines of the last message sent
    pub last_msg_sent: Option<LastMsgSent>,
}

impl PersistedState {
    pub fn load_or_inital() -> Self {
        match std::fs::read(DEFAULT_PATH.read().join(".uplink.state.json")) {
            Ok(b) => serde_json::from_slice::<PersistedState>(&b).unwrap_or_default(),
            Err(_) => Default::default(),
        }
    }

    pub fn save(&self) {
        match serde_json::to_vec(self) {
            Ok(bytes) => {
                match std::fs::write(DEFAULT_PATH.read().join(".uplink.state.json"), &bytes) {
                    Ok(_) => println!("save successful"),
                    Err(e) => eprintln!("error saving: {}", e),
                }
            }
            Err(e) => eprintln!("error serializing on save: {}", e),
        }
    }

    pub fn dispatch(&mut self, action: Actions) {
        let next = match action {
            Actions::ChatWith(info) => PersistedState {
                current_chat: Some(info.conversation.id()),
                all_chats: self.all_chats.clone(),
            },
            Actions::AddRemoveConversations(new_chats) => PersistedState {
                current_chat: self.current_chat.clone(),
                all_chats: new_chats,
            },
            Actions::UpdateConversation(info) => {
                let mut next = PersistedState {
                    current_chat: self.current_chat.clone(),
                    all_chats: self.all_chats.clone(),
                };
                // overwrite the existing entry
                next.all_chats.insert(info.conversation.id(), info);
                next
            }
        };
        // only save while there's a lock on PersistedState
        next.save();

        // modify PersistedState via assignment rather than mutation
        *self = next;
    }
}

// doesn't run when the window is closed.
// does run on the value inside of dispatch though.
// basically don't use this
//impl Drop for PersistedState {
//    fn drop(&mut self) {
//        println!("saving PersistedState");
//        self.save();
//    }
//}

impl LastMsgSent {
    pub fn new(msg: String) -> Self {
        Self {
            value: msg,
            time: Local::now(),
        }
    }

    pub fn display_time(&self) -> String {
        format!("{}:{}", self.time.hour(), self.time.minute())
    }
}
