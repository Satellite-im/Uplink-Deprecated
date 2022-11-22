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
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let identity = account.read().get_own_identity().unwrap();
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

    fn update_status_msg(
        status_msg_state: &UseState<String>,
        status_msg: &String,
        status_msg_error: &UseState<String>,
        account: &Account,
        edit_status_msg_state: &UseState<bool>,
    ) {
        let status_msg_text = status_msg_state.trim();
        if status_msg_text != *status_msg {
            if status_msg_text.len() > 128 {
                status_msg_error.set("status message needs to be less than 128 characters".into());
            } else {
                if let Err(e) = account
                    .write()
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

    cx.render(rsx! {
        div{
            label {
                "Status Message"
            },
        if **edit_status_msg_state {rsx! (
            div {
                class: "change-profile",
                div{
                    class: "input-profile",
                    Input {
                        placeholder: "Digit your status message...".to_string(),
                        value: status_msg_state.to_string(),
                        on_change: move |e: FormEvent| status_msg_state.set(e.value.clone()),
                        on_enter:move|_|{
                            update_status_msg(status_msg_state,&status_msg,status_msg_error,account,edit_status_msg_state);
                            }   
                        },
                    },
                    Button {
                        text: l.save.to_string(),
                      on_pressed: move |_|{
                          update_status_msg(status_msg_state,&status_msg2,status_msg_error,account,edit_status_msg_state);
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
