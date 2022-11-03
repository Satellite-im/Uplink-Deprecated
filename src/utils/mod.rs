use warp::{crypto::DID, multipass::identity::Identity};

use crate::{state::ConversationInfo, Account};

pub mod config;
pub mod notifications;
pub mod sounds;
pub mod get_meta;

pub fn remove_writespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

pub fn get_username_from_conversation(
    conversation_info: &ConversationInfo,
    mp: &Account,
) -> (DID, String) {
    let display_did = conversation_info
        .conversation
        .recipients()
        .last()
        .cloned()
        .unwrap_or_default();

    let display_username = get_username_from_did(display_did.clone(), mp);
    (display_did, display_username)
}

pub fn get_username_from_did(did: DID, mp: &Account) -> String {
    let display_user = mp.read().get_identity(did.into()).unwrap_or_default();
    display_user
        .first()
        .map(Identity::username)
        .unwrap_or_else(String::new)
}


pub fn get_pfp_from_did(did: DID, mp: &Account) -> String {
    let display_user = mp.read().get_identity(did.into()).unwrap_or_default();
    display_user
        .first()
        .unwrap()
        .graphics()
        .profile_picture()
}