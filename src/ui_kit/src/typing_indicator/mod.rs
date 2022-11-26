use std::collections::HashMap;

use dioxus::prelude::*;
use warp::crypto::DID;

#[inline_props]
#[allow(non_snake_case)]
pub fn TypingIndicator(cx: Scope, users: UseRef<HashMap<DID, String>>) -> Element {
    let users_list: Vec<String> = users.read().iter().map(|(_k, v)| v.clone()).collect();
    let name_typing = if users_list.len() <= 3 {
        users_list.join(", ")
    } else {
        users_list.len().to_string() + " users"
    };
    let article = if users_list.is_empty() {
        return None;
    } else if users_list.len() == 1 {
        String::from(" is")
    } else {
        String::from(" are")
    };

    cx.render(rsx! {
        div {
            class:"typing-indicator",
            div {
                class: "loader",
            }
            div {
                class: "primary",
                "{name_typing}"
            }
            div {
                class: "secondary",
                "{article} typing..."
            }
        }
    })
}
