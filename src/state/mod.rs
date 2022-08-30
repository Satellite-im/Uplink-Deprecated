use warp::crypto::DID;

use self::mutations::Mutations;

pub mod mutations;

pub enum Actions {
    ChatWith(DID)
}

pub struct PersistedState {
    pub chats: Vec<DID>,
}

impl PersistedState {
    pub fn load_or_inital() -> Self {
        Self::inital()
    }
    pub fn inital() -> Self {
        PersistedState {
            chats: vec![]
        }
    }
    pub fn dispatch(&mut self, action: Actions) -> Self {
        match action {
            Actions::ChatWith(did) => Mutations::chat_with(self, did),
        };

        PersistedState { chats: self.chats.clone() }
    }
}