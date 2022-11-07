use regex::Regex;
use warp::{crypto::DID, multipass::identity::Identity};

use crate::{state::ConversationInfo, Account};

pub mod config;
pub mod get_meta;
pub mod notifications;
pub mod sounds;

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

pub fn get_pfp_from_did(did: DID, mp: &Account) -> Option<String> {
    let display_user = mp.read().get_identity(did.into()).unwrap_or_default();
    display_user
        .first()
        .map(|ident| ident.graphics().profile_picture())
}

pub fn wrap_in_markdown(val: &str) -> String {
    let replace_newlines = val.clone().replace('\n', "<br />");

    let regex_asterisk = Regex::new(r"\*{2}(.*?)\*{2}").unwrap();
    let regex_tilda = Regex::new(r"\~{2}(.*?)\~{2}").unwrap();
    let regex_one_underscore = Regex::new(r"_{1}(.*?)_{1}").unwrap();
    let regex_two_underscores_first = Regex::new(r"_{2}(.*?)_{2}").unwrap();
    let regex_two_underscores_second = Regex::new(r"\){3}(.*?)\({3}").unwrap();

    let replacement_asterisk =
        "<span class=\"delimiter\">**</span><b>$1</b><span class=\"delimiter\">**</span>";
    let replacement_tilda =
        "<span class=\"delimiter\">~~</span><strike>$1</strike><span class=\"delimiter\">~~</span>";
    let replacement_one_underscore =
        "<span class=\"delimiter\">_</span><i>$1</i><span class=\"delimiter\">_</span>";
    let replacement_two_underscore_first = ")))$1(((";
    let replacement_two_underscore_second =
        "<span class=\"delimiter\">__</span><b>$1</b><span class=\"delimiter\">__</span>";

    let final_string = regex_asterisk.replace_all(replace_newlines.as_str(), replacement_asterisk);
    let final_string = regex_tilda.replace_all(&final_string, replacement_tilda);
    let final_string =
        regex_two_underscores_first.replace_all(&final_string, replacement_two_underscore_first);
    let final_string = regex_one_underscore.replace_all(&final_string, replacement_one_underscore);
    let final_string =
        regex_two_underscores_second.replace_all(&final_string, replacement_two_underscore_second);

    String::from(final_string)
}
