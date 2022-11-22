use dioxus::prelude::*;
use crate::{
    Account, STATE, utils
};
use uuid::Uuid;

#[derive(PartialEq, Props)]
pub struct Props {
    account: Account,
    current_chat: Uuid,
}

#[allow(non_snake_case)]
pub fn UsersTypingIndicator(cx: Scope<Props>) -> Element {
    let typing_state = use_atom_ref(&cx, STATE).clone().read().participants_status;
    let mp = cx.props.account.clone();
    let current_chat = &cx.props.current_chat.clone();
    let mut users_list: Vec<String> = Vec::new();
    if !current_chat.is_nil() {
        if typing_state.contains_key(current_chat) {
            for (&did, user) in typing_state[current_chat].iter() {
                let username = utils::get_username_from_did(did, &mp).to_string();
                if user.typing && !users_list.contains(&username) {
                    users_list.push(username);
                }
                if !user.typing && users_list.contains(&username) {
                    users_list.retain(|v| *v != username);
                }
            }
        }
    }
    

    let name_typing = if users_list.len() <= 3 {
        users_list.join(", ")
    } else {
        users_list.len().to_string() + " users"
    };
    let article = if users_list.is_empty() {
        String::from("Why do i see this indicator? None is")
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
