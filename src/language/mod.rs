pub mod en_us;

pub enum AvailableLanguages {
    EN_US
}

pub struct Language {
    pub unlock: String,
    pub unlock_title: String,
    pub unlock_desc: String,
    pub passphrase: String,
}

pub fn by_locale(lang: AvailableLanguages) -> Language {
    match lang {
        AvailableLanguages::EN_US => en_us::make()
    }
}
