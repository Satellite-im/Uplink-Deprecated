use dioxus::{events::FormEvent, prelude::*};
use regex::RegexSet;
use sir::css;
use ui_kit::{
    button::{self, Button},
    input::Input,
};
use warp::multipass::identity::IdentityUpdate;

use crate::{Account, LANGUAGE};

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}
#[inline_props]
#[allow(non_snake_case)]
pub fn Username(cx: Scope<Props>, account: Account) -> Element {
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let identity = account.read().get_own_identity().unwrap();
    let username = identity.username();
    let username2 = username.clone();
    let username_state = use_state(&cx, || username.clone());
    let edit_username_state = use_state(&cx, || false);
    let username_error = use_state(&cx, String::new);
    let username_error_class = if username_error.is_empty() {
        css!("opacity: 0")
    } else {
        "error_text"
    };

    fn update_username(
        username_state: &UseState<String>,
        username: &String,
        username_error: &UseState<String>,
        account: &Account,
        edit_username_state: &UseState<bool>,
    ) {
        let username_text = username_state.trim();
        if username_text != *username {
            if username_text.is_empty() {
                username_error.set("needs change here".into())
            } else if username_text.len() < 4 || username_text.len() > 32 {
                username_error.set("Username needs to be between 4 and 32 characters long".into())
            } else {
                let username_regex_set =
                    RegexSet::new(&[r"@", r"[[:^alnum:]&&[:^punct:]]"]).unwrap();
                let matches = username_regex_set.matches(username_text);
                if matches.matched(0) {
                    username_error.set("@ is not allowed in username".into());
                } else if matches.matched(1) {
                    username_error.set("Illegal input in username".into());
                } else {
                    if let Err(e) = account
                        .write()
                        .update_identity(IdentityUpdate::set_username(username_text.to_string()))
                    {
                        println!("Failed in updating status message:{}", e);
                    }
                    edit_username_state.set(false);
                }
            }
        } else {
            edit_username_state.set(false);
        }
    }

    cx.render(rsx!{
        div{
            label {
                "User Name"
            },
            if **edit_username_state {rsx! (
                div {
                    class: "change-profile",
                    div{
                        class: "input-profile",
                        Input {
                            placeholder: "Digit your username...".to_string(),
                            value: username_state.to_string(),
                            on_change: move |e: FormEvent| {
                                username_error.set("".into());
                                username_state.set(e.value.clone());
                            },
                            on_enter:move|_|{
                                update_username(username_state,&username,username_error,account,edit_username_state,);
                    },
                },
            },
            Button {
                        text: l.save.to_string(),
                        on_pressed: move |_|{
                            update_username(username_state,&username2,username_error,account,edit_username_state, );
                        }
                    }
                },
            )} else {rsx! (
                div{
                    class: "change-profile",
                    span {
                        "{username}"
                },
                   Button {
                            text: l.edit.to_string(),
                            state: button::State::Secondary,
                            on_pressed: move |_| {
                                edit_username_state.set(true);
                            },
                    },
                },)
            },
            p {
                class: "{username_error_class}",
                "{username_error}"
            },
        }

    })
}
