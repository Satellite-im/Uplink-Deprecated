use super::{PersistedState, Conversation};

pub struct Mutations;
impl Mutations {
    pub fn chat_with(state: &mut PersistedState, conversation: Conversation) {
        let mut chats = state.chats.clone();
        let already_there = chats.iter().any(|c| c.id.to_string() == conversation.id.to_string());
        if already_there {
            let index = chats.iter().position(|x| x.id.to_string() == conversation.id.to_string());
            match index {
                Some(i) => {
                    chats.remove(i);
                }
                None => {}
            };
        }
        chats.push(conversation.clone());
        state.chats = chats;
        state.chat = Some(conversation.clone());
    }
}
