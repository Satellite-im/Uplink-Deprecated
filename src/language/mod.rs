pub mod en_us;

pub enum AvailableLanguages {
    EnUS,
}

#[derive(Clone)]
pub struct Language {
    pub couldnt_send: String,
    pub incoming_requests: String,
    pub outgoing_requests: String,
    pub your_friends: String,
    pub already_sent: String,
    pub add_self: String,
    pub friend_exist: String,
    pub something_went_wrong: String,
    pub create_pin: String,
    pub enter_pin: String,
    pub enter_your_pin: String,
    pub choose_a_pin: String,
    pub user_agrees: String,
    pub prerelease_warning: String,
    pub send_a_reply: String,
    pub invalid_pin: String,
    pub short_pin: String,
    pub checking_account: String,
    pub create_account: String,
    pub create_account_desc: String,
    pub choose_username: String,
    pub chatbar_placeholder: String,
    pub chat_placeholder: String,
    pub copy_friend_code: String,
    pub copy_code: String,
    pub code_copied: String,
    pub copied_code: String,
    pub add_someone: String,
    pub add_placeholder: String,
    pub invalid_code: String,
    pub request_sent: String,
    pub about: String,
    pub unknown: String,
    pub location: String,
    pub badges: String,
    pub save: String,
    pub edit: String,
    pub username: String,
    pub username_placeholder: String,
    pub username_error_required: String,
    pub username_error_length: String,
    pub username_error_at_sign: String,
    pub username_error_illegal: String,
    pub status_msg: String,
    pub status_placeholder: String,
    pub status_error_length: String,
    pub friends: String,
    pub edit_profile: String,
    pub no_about_message: String,
    pub development: String,
    pub search: String,
    pub favorites: String,
    pub new_chat: String,
    pub chats: String,
    pub no_active_chats: String,
    pub start_one: String,
    pub auth_tooltip: String,
    pub new_friend_request: String,
}

impl Language {
    pub fn by_locale(lang: AvailableLanguages) -> Language {
        match lang {
            AvailableLanguages::EnUS => en_us::make(),
        }
    }
}
