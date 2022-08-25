pub mod en_us;

pub enum AvailableLanguages {
    EN_US
}

pub struct Language {
    pub create_pin: String,
}

impl Language {
    pub fn by_locale(lang: AvailableLanguages) -> Language {
        match lang {
            AvailableLanguages::EN_US => en_us::make()
        }
    }
}
