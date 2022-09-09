use warp::raygun::Conversation;

use super::PersistedState;

pub struct Mutations;
impl Mutations {
    pub fn chat_with(state: &mut PersistedState, conversation: Conversation) {
        let c = conversation.clone();
        let chats = state.chats.clone();

        for (i, chat) in state.chats.read().clone().iter().enumerate() {
            
            let mut recipients_equal = true;
            for recipient in chat.recipients().clone() {
                if !c.recipients().contains(&recipient) {
                    recipients_equal = false;
                    break;
                }
            }

            if recipients_equal {
                chats.write().remove(i);
            }
        }
        chats.write().push(conversation.clone());
        state.chats = chats;
        *state.chat.write() = Some(conversation.clone());
    }
}
