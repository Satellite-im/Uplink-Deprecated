use serde::{Serialize, Deserialize};
use warp::crypto::DID;

use self::mutations::Mutations;

pub mod mutations;

const STORAGE_LOCATION: &str = "./.warpgui.state.json";

pub enum Actions {
    ChatWith(DID)
}

#[derive(Serialize, Deserialize)]
pub struct PersistedState {
    pub chats: Vec<DID>,
}

impl PersistedState {
    pub fn load_or_inital() -> Self {
        let bytes = std::fs::read("/path/to/state");
        match bytes {
            Ok(b) => { 
                match serde_json::from_slice::<PersistedState>(&b) {
                    Ok(s) => {
                       s.chats
                    },
                    Err(_) => Self::inital(),
                }
            },
            Err(_) => Self::inital(),
        }
        
    }

    pub fn save(&self) {
        let bytes = serde_json::to_vec(self.into());
        match bytes {
            Ok(b) => { 
                let _ = std::fs::write(STORAGE_LOCATION, &b);
            },
            Err(_) => {},
        }
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