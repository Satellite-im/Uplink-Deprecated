use super::{ConversationInfo, Conversations};

pub struct Mutations;
impl Mutations {
    pub fn chat_with(conversations: &mut Conversations, conversation_info: ConversationInfo) {
        // todo: revisit this behavior. seems to make things hard for the user
        unimplemented!()
        /*let mut all_chats: Vec<ConversationInfo> = conversations
            .all_chats
            .iter()
            .filter(|current| current.conversation.id() != conversation_info.conversation.id())
            .cloned()
            .collect();
        all_chats.push(conversation_info.clone());

        conversations.all_chats = all_chats;
        conversations.current_chat = Some(conversation_info);*/
    }
}
