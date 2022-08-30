use warp::crypto::DID;

use super::PersistedState;

pub struct Mutations;
impl Mutations {
    pub fn chat_with(state: &mut PersistedState, did: DID) {
        let mut chats = state.chats.clone();
        let already_there = chats.iter().any(|d| { d.to_string() == did.to_string()});
        if already_there {
            let index = chats.iter().position(|x| x.to_string() == did.to_string());
            match index {
                Some(i) => {
                    chats.remove(i);
                },
                None => {},
            };
        }
        chats.push(did.clone());
        state.chats = chats;
        state.chat = Some(did.clone());
    }
}