use warp::raygun::Conversation;

use super::Conversations;

pub struct Mutations;
impl Mutations {
    pub fn chat_with(state: &mut Conversations, conversation: Conversation) {
        let c = conversation.clone();
        let mut chats = state.all_chats.clone();

        for (i, chat) in state.all_chats.clone().iter().enumerate() {
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
        state.all_chats = chats;
        state.current_chat = Some(conversation);
    }
}
