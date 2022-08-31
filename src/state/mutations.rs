use super::{PersistedState, Conversation};

pub struct Mutations;
impl Mutations {
    pub fn chat_with(state: &mut PersistedState, conversation: Conversation) {
        let mut chats = state.chats.clone();
        let already_there = chats.iter().any(|d| d.id == conversation.id);
        if already_there {
            let index = chats.iter().position(|x| x.id == conversation.id);
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
