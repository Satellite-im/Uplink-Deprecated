use warp::raygun::Conversation;

use super::PersistedState;

pub struct Mutations;
impl Mutations {
    pub fn chat_with(state: &mut PersistedState, conversation: Conversation) {
        let c = conversation.clone();
        let mut chats = state.chats.clone();

        for (i, chat) in state.chats.clone().iter().enumerate() {
            
            let mut recipients_equal = true;
            for recipient in chat.recipients().clone() {
                if !c.recipients().contains(&recipient) {
                    recipients_equal = false;
                    break;
                }
            }

            if recipients_equal {
                chats.remove(i);
            }
        }
        chats.push(conversation.clone());
        state.chats = chats;
        state.chat = Some(conversation);
    }
}
