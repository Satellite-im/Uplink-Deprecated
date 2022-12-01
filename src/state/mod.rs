use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    cmp::{Ord, Ordering},
    collections::{HashMap, HashSet},
};
// use utils::{notifications::PushNotification, sounds::Sounds};
use uuid::Uuid;
use warp::raygun::Conversation;

use crate::DEFAULT_PATH;

pub enum Actions {
    AddConversation(Conversation),
    ChatWith(ConversationInfo),
    UpdateConversation(ConversationInfo),
    UpdateFavorites(HashSet<Uuid>),
    HideSidebar(bool),
    RemoveChat(ConversationInfo),
    ClearChat,
    SetShowPrerelaseNotice(bool),
    // SendNotification(String, String, Sounds),
}

/// tracks the active conversations. Chagnes are persisted
#[derive(Serialize, Deserialize, Default, Eq, PartialEq, Clone)]
pub struct PersistedState {
    /// the currently selected conversation
    pub current_chat: Option<ConversationInfo>,
    /// all active conversations
    pub all_chats: HashMap<Uuid, ConversationInfo>,
    pub hidden_chat: HashSet<Uuid>,
    /// a list of favorited conversations.
    /// Uuid is for Conversation and can be used to look things up in all_chats
    pub favorites: HashSet<Uuid>,
    // show sidebar boolean, used with in mobile view
    pub hide_sidebar: bool,
    pub total_unreads: u32,
    pub show_prerelease_notice: bool,
}

#[derive(Serialize, Deserialize, Default, Clone, Eq, PartialEq)]
pub struct LastMsgSent {
    pub value: String,
    pub time: DateTime<Utc>,
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
    pub creation_time: DateTime<Utc>,
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

pub fn total_notifications(s: &PersistedState) -> u32 {
    let mut count = 0;
    for convo in s.all_chats.iter() {
        let convo_count = convo.1.clone().num_unread_messages;
        count += convo_count;
    }
    count
}

impl PersistedState {
    pub fn load_or_initial() -> Self {
        match std::fs::read(DEFAULT_PATH.read().join(".uplink.state.json")) {
            Ok(b) => serde_json::from_slice::<PersistedState>(&b).unwrap_or_default(),
            Err(_) => {
                let mut state: PersistedState = Default::default();
                state.show_prerelease_notice = true;
                state
            }
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
        match action {
            Actions::AddRemoveConversations(new_chats) => {
                self.favorites = self
                    .favorites
                    .iter()
                    .filter(|id| conversation.id() == **id) //Note: this might need to be changed
                    .cloned()
                    .collect();
                self.all_chats = new_chats;
                self.total_unreads = total_notifications(&self);
            }
            Actions::ClearChat => {
                self.current_chat = None;
            }
            Actions::ChatWith(info) => {
                self.current_chat = Some(info.conversation.id());
            }
            Actions::UpdateConversation(info) => {
                self.all_chats.insert(info.conversation.id(), info);
                self.total_unreads = total_notifications(&self);
            }
            Actions::UpdateFavorites(favorites) => {
                self.favorites = favorites;
            }
            Actions::HideSidebar(slide_bar_bool) => {
                self.hide_sidebar = slide_bar_bool;
            }
            Actions::RemoveChat(info) => {
                let uuid = info.conversation.id();
                self.all_chats.remove(&uuid);
                // If the current chat was set to this, we'll want to remove that too.
                self.current_chat = match next.current_chat {
                    Some(u) => {
                        if u.conversation.id().eq(&uuid) {
                            None
                        } else {
                            next.current_chat
                        }
                    }
                    None => None,
                };
            }
            Actions::SetShowPrerelaseNotice(value) => {
                self.show_prerelease_notice = value;
            } // Actions::SendNotification(title, content, sound) => {
              //     let _ = PushNotification(title, content, sound);
              //     PersistedState {
              //         current_chat: self.current_chat,
              //         all_chats: self.all_chats.clone(),
              //         favorites: self.favorites.clone(),
              //         hide_sidebar: self.hide_sidebar,
              //         total_unreads: total_notifications(&self),
              //     }
              // }
        };
        self.save();
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
            time: DateTime::from(Local::now()),
        }
    }
}
