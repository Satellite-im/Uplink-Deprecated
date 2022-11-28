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
    let mut account = account.clone();
    let mut account2 = account.clone();
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let l2 = l.clone();
    let l3 = l.clone();
    let identity = account.get_own_identity().unwrap();
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
    cx.render(rsx!{
        div{
            label {
                "{l.username}"
            },
            if **edit_username_state {rsx! (
                div {
                    class: "change-profile",
                    div{
                        class: "input-profile",
                        Input {
                            placeholder: l.username_placeholder.to_string(),
                            value: username_state.to_string(),
                            on_change: move |e: FormEvent| {
                                username_error.set("".into());
                                username_state.set(e.value.clone());
                            },
                            on_enter:move|_|{
                                let username_text = username_state.trim();
                                if username_text != username {
                                    if username_text.is_empty() {
                                        username_error.set(l2.username_error_required.to_string())
                                    } else if username_text.len() < 4 || username_text.len() > 32 {
                                         username_error.set(l2.username_error_length.to_string())
                                    } else {
                                        let username_regex_set =
                                            RegexSet::new(&[r"@", r"[[:^alnum:]&&[:^punct:]]&&^ "]).unwrap();
                                        let matches = username_regex_set.matches(username_text);
                                        if matches.matched(0) {
                                            username_error.set(l2.username_error_at_sign.to_string())
                                        } else if matches.matched(1){
                                            username_error.set(l2.username_error_illegal.to_string())
                                        } else {
                                            if let Err(e) = account
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
                    },
                },
            },
            Button {
                        text: l.save.to_string(),
                        on_pressed: move |_|{
                            let username_text = username_state.trim();
                            if username_text != username2 {
                                if username_text.is_empty() {
                                    username_error.set(l3.username_error_required.to_string())
                                } else if username_text.len() < 4 || username_text.len() > 32 {
                                     username_error.set(l3.username_error_length.to_string())
                                } else {
                                    let username_regex_set =
                                        RegexSet::new(&[r"@", r"[[:^alnum:]&&[:^punct:]]"]).unwrap();
                                    let matches = username_regex_set.matches(username_text);
                                    if matches.matched(0) {
                                        username_error.set(l3.username_error_at_sign.to_string())
                                    } else if matches.matched(1){
                                        username_error.set(l3.username_error_illegal.to_string())
                                    } else {
                                        if let Err(e) = account2
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
