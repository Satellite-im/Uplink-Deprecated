use dioxus::prelude::*;

use crate::{Account, LANGUAGE};
use dioxus::events::FormEvent;
use ui_kit::{
    button::{self, Button},
    input::Input,
    photo_picker::PhotoPicker,
};
use warp::multipass::identity::IdentityUpdate;

#[derive(Props, PartialEq)]
pub struct Props {
    account: Account,
}

#[inline_props]
#[allow(non_snake_case)]
pub fn Profile(cx: Scope<Props>, account:Account) -> Element {
    log::debug!("rendering settings/pages/Profile");
    let l = use_atom_ref(&cx, LANGUAGE).read();
    let l2= l.clone();

    let account2 = account.clone();
    let identity = account2.read().get_own_identity().unwrap();

    let user_name = identity.username();
    let user_name_state = use_state(&cx, || user_name.clone());
    let edit_user_name_state = use_state(&cx, || false);

    let status_msg = identity.status_message();
    let status_msg_state = use_state(&cx, || match status_msg.clone() {
        Some(msg) => msg,
        None => String::new(),
    });
    let edit_status_msg_state = use_state(&cx, || false);

    let update_user_name = move |_| {
        // user name can't be none
        // unfinished
        edit_user_name_state.set(false);
    };

    let update_status_msg = move |_| {
        if let Err(e) =  account
          .write()
          .update_identity(IdentityUpdate::set_status_message(Some(
              status_msg_state.to_string(),
          )))
      {
          println!("Failed in updating status message:{}", e);
      }
      edit_status_msg_state.set(false);
  };

    cx.render(rsx! {
        div {
            id: "page_profile",
            class: "padded",
            div {
                class: "profile-header",
                div {
                    class: "profile-picture",
                    PhotoPicker {
                        account: cx.props.account.clone(),
                    },
                }
            },
            div {
                div{
                    label {
                        "User Name"
                    },
                    if **edit_user_name_state {rsx! (
                        div {
                            class: "change-profile",
                            div{
                                class: "input-profile",
                                Input {
                                    placeholder: "type user name".to_string(),
                                    // value: user_name_state.to_string(),
                                    on_change: move |e: FormEvent| user_name_state.set(e.value.clone()),
                                    on_enter:move|_|{},
                                },
                            },         
                            Button {
                                text: l.save.to_string(),
                                on_pressed: update_user_name
                            }
                        },
                    )} else {rsx! (
                        div{
                            class: "change-profile",
                             span {
                            "username"
                        },
                           Button {
                                    text: l.edit.to_string(),
                                    state: button::State::Secondary,
                                    on_pressed: move |_| {
                                        edit_user_name_state.set(true);
                                    },
                            },
                        },)
                    },
                }
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
                                placeholder: "type".to_string(),
                                // value: status_msg_state.to_string(),
                                on_change: move |e: FormEvent| status_msg_state.set(e.value.clone()),
                                on_enter:move|_|{},
                            },
                        },              
                        Button {
                          text: l2.save.to_string(),
                        on_pressed: update_status_msg
                        }
                    },
                )} else {rsx! (
                    div{
                        class: "change-profile",
                        span {
                            status_msg
                        },
                       Button {
                        text: l2.edit.to_string(),
                        state: button::State::Secondary,
                                on_pressed: move |_| {
                                    edit_status_msg_state.set(true);
                                },
                            },
                        },)
                }}
            },
        }
    })
}
