use chrono::prelude::*;
use chrono_humanize::HumanTime;
use serde::{Deserialize, Serialize};
use std::{
    cmp::{Ord, Ordering},
    collections::{HashMap, HashSet},
};
use uuid::Uuid;
use warp::raygun::Conversation;

use crate::DEFAULT_PATH;

pub enum Actions {
    AddRemoveConversations(HashMap<Uuid, ConversationInfo>),
    ChatWith(ConversationInfo),
    UpdateConversation(ConversationInfo),
    UpdateFavorites(HashSet<Uuid>),
}

/// tracks the active conversations. Chagnes are persisted
#[derive(Serialize, Deserialize, Default, Eq, PartialEq)]
pub struct PersistedState {
    /// the currently selected conversation
    pub current_chat: Option<Uuid>,
    /// all active conversations
    pub all_chats: HashMap<Uuid, ConversationInfo>,
    /// a list of favorited conversations.
    /// Uuid is for Conversation and can be used to look things up in all_chats
    pub favorites: HashSet<Uuid>,
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
    /// the uuid of the last message read.
    /// used to determine the number of unread messages
    pub num_unread_messages: u32,
    /// the first two lines of the last message sent
    pub last_msg_sent: Option<LastMsgSent>,
    /// the time the conversation was created. used to sort the chats
    pub creation_time: DateTime<Local>,
}

impl Ord for ConversationInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        // partial_cmp never returns None, but if it did, comparing by name is the next best thing.
        self.partial_cmp(other)
            .unwrap_or_else(|| self.conversation.name().cmp(&other.conversation.name()))
    }
}

impl PartialOrd for ConversationInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let left = match &self.last_msg_sent {
            Some(left) => left.time,
            None => self.creation_time,
        };

        let right = match &other.last_msg_sent {
            Some(right) => right.time,
            None => other.creation_time,
        };

        Some(left.cmp(&right))
    }
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
                    Ok(_) => {}
                    Err(e) => eprintln!("error saving: {}", e),
                }
            }
            Err(e) => eprintln!("error serializing on save: {}", e),
        }
    }

    pub fn dispatch(&mut self, action: Actions) {
        let next = match action {
            Actions::AddRemoveConversations(new_chats) => {
                let favorites = self
                    .favorites
                    .iter()
                    .filter(|id| new_chats.contains_key(id))
                    .cloned()
                    .collect();

                PersistedState {
                    current_chat: self.current_chat,
                    all_chats: new_chats,
                    favorites,
                }
            }
            Actions::ChatWith(info) => PersistedState {
                current_chat: Some(info.conversation.id()),
                all_chats: self.all_chats.clone(),
                favorites: self.favorites.clone(),
            },
            Actions::UpdateConversation(info) => {
                let mut next = PersistedState {
                    current_chat: self.current_chat,
                    all_chats: self.all_chats.clone(),
                    favorites: self.favorites.clone(),
                };
                // overwrite the existing entry
                next.all_chats.insert(info.conversation.id(), info);
                next
            }
            Actions::UpdateFavorites(favorites) => PersistedState {
                current_chat: self.current_chat,
                all_chats: self.all_chats.clone(),
                favorites,
            },
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
    pub fn new(msg: &[String]) -> Self {
        Self {
            // the sizing of the conversation box is fixed, so approximate the needed string length using
            // the placeholder text
            value: msg
                .iter()
                .take(2)
                .cloned()
                .collect::<Vec<String>>()
                .join("\n")
                .chars()
                .take(24)
                .collect(),
            time: Local::now(),
        }
    }

    pub fn display_time(&self) -> String {
        let ht = HumanTime::from(self.time);
        let s = ht.to_string();
        let mut split = s.split(char::is_whitespace);
        let time = split.next().unwrap_or("");
        let units = split.next().unwrap_or("").chars().next().unwrap_or(' ');
        // TODO: this might not be ideal to support multiple locales.
        format!("{}{}", time, units)
    }
}
