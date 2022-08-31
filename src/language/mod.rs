pub mod en_us;

pub enum AvailableLanguages {
    EnUS,
}

#[derive(Clone)]
pub struct Language {
    pub create_pin: String,
    pub enter_pin: String,
    pub enter_your_pin: String,
    pub choose_a_pin: String,
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
    pub copied_code: String,
    pub add_someone: String,
    pub add_placeholder: String,
    pub invalid_code: String,
    pub request_sent: String,
}

impl Language {
    pub fn by_locale(lang: AvailableLanguages) -> Language {
        match lang {
            AvailableLanguages::EnUS => en_us::make(),
        }
    }
}
