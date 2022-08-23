use super::Language;

pub fn make() -> Language {
    Language {
        unlock: "Unlock".to_string(),
        unlock_title: "Unlock Your Account".to_string(),
        unlock_desc: "This password is used to secure all of the data inside your account, choose wisely and don't forget it.".to_string(),
        passphrase: "Passphrase".to_string(),

    }
}