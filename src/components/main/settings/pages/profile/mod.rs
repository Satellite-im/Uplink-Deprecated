use dioxus::prelude::*;
use regex::RegexSet;
use sir::css;
use crate::{Account, LANGUAGE};
use dioxus::events::FormEvent;
use ui_kit::{
    button::{self, Button},
    input::Input,
    photo_picker::PhotoPicker,
};
use warp::multipass::identity::{IdentityUpdate, Identifier};

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
    let user_name_error = use_state(&cx, String::new);
    let user_name_error_class = if user_name_error.is_empty() {
        css!("opacity: 0")
    } else {
            "error_text"
        };


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
                                    value: user_name_state.to_string(),
                                    on_change: move |e: FormEvent| {
                                        user_name_error.set("".into());
                                        user_name_state.set(e.value.clone());
                                    },
                                    on_enter:move|_|{
                                let user_name_text = user_name_state.trim();

                                if user_name_text != user_name{
                                        if user_name_text.is_empty(){
                                                user_name_error.set("Username is required".into())
                                        }else if user_name_text.len() < 4 ||user_name_text.len() > 32  {
                                                user_name_error.set("Username needs to be between 4 and 32 characters long".into())
                                        }else{
                                        let user_name_regex_set = RegexSet::new(&[
                                                    r"@",
                                                    r"\p{Emoji}",
                                         ]).unwrap();
                                                    
                                        let matches: Vec<_> = user_name_regex_set.matches(user_name_text).into_iter().collect();

                                        if matches.contains(&0){
                                            user_name_error.set("@ is not allowed in username".into());
                                        } else if matches.contains(&1){
                                            user_name_error.set("emoji is not allowed in username".into());
                                        } else {
                                        //TODO: check if username already exists
                                          let return_result =  account.write().get_identity(Identifier::user_name(user_name_text));

                                         match return_result {
                                            Ok(user) => println!("user: {:?}",user.iter()),
                                            Err(e) => println!("error: {}",e),
                                         }
                                            // if let Err(e) =  account.write().update_identity(IdentityUpdate::set_username(
                                            //     user_name_text.to_string()))
                                            //     {
                                            //     println!("Failed in updating status message:{}", e);
                                            //     }
                                            //     edit_user_name_state.set(false);
                                        }
                                    }
                                }
                            },
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
                                "{user_name}"
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
                    p {
                        class: "{user_name_error_class}",
                        "{user_name_error}"
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
                                value: status_msg_state.to_string(),
                                on_change: move |e: FormEvent| status_msg_state.set(e.value.clone()),
                                on_enter:move|_|{
                                    //TODO: add if msg changed here and on_pressed
                                    if let Err(e) =  account.write().update_identity(IdentityUpdate::set_status_message(
                                        Some(status_msg_state.to_string(),)))
                                        {
                                        println!("Failed in updating status message:{}", e);
                                        }
                                    edit_status_msg_state.set(false);
                                },
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
