use super::Language;

pub fn make() -> Language {
    Language {
        create_pin: "Create a Pin".to_string(),
        enter_pin: "Enter Pin".to_string(),
        enter_your_pin: "Enter pin to unlock your account.".to_string(),
        choose_a_pin: "Choose a 4-6 diget pin to secure your account.".to_string(),
        invalid_pin: "Invalid or incorrect pin supplied.".to_string(),
        short_pin: "Your pin must be at least 4 characters.".to_string(),
        checking_account: "Checking account..".to_string(),
        create_account: "Create Account".to_string(),
        create_account_desc:
            "It's free and fast, just tell us what you'd like your username to be.".to_string(),
        choose_username: "Choose username".to_string(),
        chatbar_placeholder: "Say something...".to_string(),
        chat_placeholder: "It's quiet... click here to start this convorsation.".to_string(),
        copy_friend_code: "Copy Your Friend Code".to_string(),
        copy_code: "Copy Code".to_string(),
        copied_code: "Friend code copied!".to_string(),
        add_someone: "Add Someone".to_string(),
        add_placeholder: "Warp#a3fdc6..".to_string(),
        request_sent: "Friend request sent!".to_string(),
        invalid_code: "Invalid friend code supplied".to_string(),
    }
}
