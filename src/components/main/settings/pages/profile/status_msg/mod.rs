use dioxus::{events::FormEvent, prelude::*};
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
pub fn StatusMsg(cx: Scope<Props>, account: Account) -> Element {
    let mut account = account.clone();
    let mut account2 = account.clone();
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let l2 = l.clone();
    let l3 = l.clone();
    let identity = account.get_own_identity().unwrap();
    let status_msg = match identity.status_message() {
        Some(msg) => msg,
        None => String::new(),
    };
    let status_msg2 = status_msg.clone();
    let status_msg_state = use_state(&cx, || status_msg.clone());
    let edit_status_msg_state = use_state(&cx, || false);
    let status_msg_error = use_state(&cx, String::new);
    let status_msg_error_class = if status_msg_error.is_empty() {
        css!("opacity: 0")
    } else {
        "error_text"
    };

    cx.render(rsx! {
        div{
            label {
                "{l.status_msg}"
            },
        if **edit_status_msg_state {rsx! (
            div {
                class: "change-profile",
                div{
                    class: "input-profile",
                    Input {
                        placeholder: l.status_placeholder.to_string(),
                        value: status_msg_state.to_string(),
                        on_change: move |e: FormEvent| {
                            status_msg_error.set("".into());
                            status_msg_state.set(e.value.clone())},
                        on_enter:move |_|{
                            let status_msg_text = status_msg_state.trim();
                            if status_msg_text != status_msg {
                                if status_msg_text.len() > 128 {
                                    status_msg_error.set(l2.status_error_length.to_string());
                                } else {
                                    if let Err(e) = account
                                        .update_identity(IdentityUpdate::set_status_message(Some(
                                            status_msg_state.to_string(),
                                        )))
                                    {
                                        println!("Failed in updating status message:{}", e);
                                    }
                                    edit_status_msg_state.set(false);
                                }
                            } else {
                                edit_status_msg_state.set(false);
                            }
                        }
                    },
                },
                Button {
                    text: l.save.to_string(),
                    on_pressed: move |_|{
                        let status_msg_text = status_msg_state.trim();
                        if status_msg_text != status_msg2 {
                            if status_msg_text.len() > 128 {
                                status_msg_error.set(l3.status_error_length.to_string());
                            } else {
                                if let Err(e) = account2
                                    .update_identity(IdentityUpdate::set_status_message(Some(
                                        status_msg_state.to_string(),
                                    )))
                                {
                                    println!("Failed in updating status message:{}", e);
                                }
                                edit_status_msg_state.set(false);
                            }
                        } else {
                            edit_status_msg_state.set(false);
                        }
                    }
                },
            },
        )} else {rsx! (
            div{
                class: "change-profile",
                span {
                    "{status_msg}",
                },
               Button {
                text: l.edit.to_string(),
                state: button::State::Secondary,
                        on_pressed: move |_| {
                            edit_status_msg_state.set(true);
                        },
                    },
                },)
        }
        p {
            class: "{status_msg_error_class}",
            "{status_msg_error}"
        },}
    })
}
